#[test]
fn issue_218() {
    // https://github.com/librasn/rasn/issues/218
    let mut set_of = rasn::types::SetOf::new();
    set_of.insert(String::from("c"));
    set_of.insert(String::from("ab"));
    let encoding = rasn::der::encode(&set_of).unwrap();
    //                                    c          a   b
    //                                    |          |   |
    //                                    v          v   v
    assert_eq!(&encoding, &[49, 7, 12, 1, 99, 12, 2, 97, 98]);
}
