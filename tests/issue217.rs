use rasn::prelude::*;

#[derive(Debug, Clone, AsnType, Encode, Decode)]
#[rasn(choice)]
enum Tx {
    #[rasn(tag(explicit(0)))]
    Test,
    #[rasn(tag(explicit(1)))]
    Reset {
        #[rasn(tag(explicit(0)))]
        keys: Option<Vec<u32>>,
    }
}

#[test]
fn issue_217() {
    let tx = Tx::Reset { keys: None };
    let encoded = rasn::der::encode(&tx).unwrap();
    assert_eq!(encoded, vec![161, 2, 48, 0])
}
