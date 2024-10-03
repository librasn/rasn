#[test]
fn test_snmp() {
    let bytes: Vec<i8> = vec![
        48, -127, -125, 2, 1, 3, 48, 16, 2, 3, 0, -15, -36, 2, 3, 0, -1, -1, 4, 1, 5, 2, 1, 3, 4,
        55, 48, 53, 4, 17, -128, 0, 31, -120, -128, 101, 74, 97, 51, 95, 18, -59, 100, 0, 0, 0, 0,
        2, 1, 11, 2, 2, 106, 24, 4, 9, 98, 111, 111, 116, 115, 116, 114, 97, 112, 4, 12, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 48, 51, 4, 17, -128, 0, 31, -120, -128, 101, 74, 97, 51,
        95, 18, -59, 100, 0, 0, 0, 0, 4, 0, -96, 28, 2, 4, 65, 85, -7, -101, 2, 1, 0, 2, 1, 0, 48,
        14, 48, 12, 6, 8, 43, 6, 1, 2, 1, 1, 5, 0, 5, 0,
    ];
    let bytes: Vec<u8> = unsafe { std::mem::transmute(bytes) };
    println!("{}", {
        let mut s = String::with_capacity(bytes.len() * 2);
        for byte in &bytes {
            use std::fmt::Write;
            write!(&mut s, "{:02X}", byte).unwrap();
        }
        s
    });
    let msg: rasn_snmp::v3::Message = rasn::der::decode(&bytes).unwrap();
    let bytes2 = rasn::der::encode(&msg).unwrap();
    println!("{}", {
        let mut s = String::with_capacity(bytes2.len() * 2);
        for byte in &bytes2 {
            use std::fmt::Write;
            write!(&mut s, "{:02X}", byte).unwrap();
        }
        s
    });
    pretty_assertions::assert_eq!(bytes, bytes2);
}
