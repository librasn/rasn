#[macro_use]
extern crate afl;

fn main() {
    afl::fuzz!(|data: &[u8]| {
        // fuzz::fuzz_pkix(data);
        fuzz::fuzz_oer(data);
    });
}
