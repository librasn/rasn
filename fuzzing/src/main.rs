#[macro_use]
extern crate afl;

fn main() {
    afl::fuzz!(|data: &[u8]| {
        fuzz::fuzz(data);
    });
}
