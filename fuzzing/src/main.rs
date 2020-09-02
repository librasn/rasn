#[cfg(not(test))]
#[macro_use]
extern crate afl;

#[cfg(not(test))]
fn main() {
    fuzz!(|data: &[u8]| {
        let _ = rasn::ber::decode::<rasn::types::Open>(data);
    });
}

#[cfg(test)]
fn main() {}

#[cfg(test)]
mod tests {
    use rasn::types::Open;

    #[test]
    fn havoc1() {
        let _ = rasn::ber::decode::<bool>(&std::fs::read("data/havoc1.bin").unwrap());
    }

    #[test]
    fn havoc2() {
        let _ = rasn::ber::decode::<Open>(&std::fs::read("data/havoc2.bin").unwrap());
    }

    #[test]
    fn flip1() {
        let _ = rasn::ber::decode::<Open>(&std::fs::read("data/flip1.bin").unwrap());
    }
}
