use rasn::prelude::*;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

thread_local! {
    static TRACK: Cell<bool> = const { Cell::new(false) };
    static MAX_ALLOCATION: Cell<usize> = const { Cell::new(0) };
}

struct CountingAllocator;
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        TRACK.with(|track| {
            if track.get() {
                MAX_ALLOCATION.with(|max| max.set(max.get().max(layout.size())));
            }
        });
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        TRACK.with(|track| {
            if track.get() {
                MAX_ALLOCATION.with(|max| max.set(max.get().max(size)));
            }
        });
        unsafe { System.realloc(ptr, layout, size) }
    }
}
#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

#[derive(AsnType)]
// Only the element size matters: decoding always rejects it before construction.
struct Rejected(#[allow(dead_code)] [u8; 96]);
impl Decode for Rejected {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        _: Tag,
        _: &Constraints,
    ) -> Result<Self, D::Error> {
        use rasn::de::Error;
        Err(D::Error::custom("invalid element", decoder.codec()))
    }
}

#[derive(AsnType, Decode)]
#[rasn(delegate, size("1..=65535"))]
struct ClaimedList(#[allow(dead_code)] SequenceOf<Rejected>);

#[test]
fn invalid_first_element_does_not_preallocate_the_claimed_count() {
    // Count 65535, followed by 8 KiB of unusable input. The remaining bits
    // are not a bound on the size of a decoded element.
    let mut input = vec![0xff; 8194];
    input[1] = 0xfe;
    MAX_ALLOCATION.with(|max| max.set(0));
    TRACK.with(|track| track.set(true));
    let result = rasn::aper::decode::<ClaimedList>(&input);
    TRACK.with(|track| track.set(false));
    let max = MAX_ALLOCATION.with(Cell::get);
    assert!(result.is_err());
    assert!(
        max < 8192,
        "allocated {max} bytes before decoding any element"
    );
}
