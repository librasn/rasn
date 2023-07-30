#[test]
fn check_message() {
    let input = hex::decode("303d020103301002041d750e01020205c00401040201030410300e0400020100020100040004000400301404000400a00e02042edfbfdc0201000201003000").unwrap();
    let _: rasn_snmp::v3::Message = rasn::ber::decode(&input).unwrap();
}
