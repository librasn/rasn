#[cfg(not(test))]
#[macro_use] extern crate afl;

#[cfg(not(test))]
fn main() {
    fuzz!(|data: &[u8]| {
        let _ = rasn::ber::decode::<bool>(data);
    });
}

#[cfg(test)]
fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn havoc1() {
        let _ = rasn::ber::decode::<bool>(&std::fs::read("data/havoc1.bin").unwrap());
    }
}
