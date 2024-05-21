use rasn::types::ObjectIdentifier;

#[test]
fn issue222() {
    let arr: &[u32] = &[1, 2, 3];
    let oid = ObjectIdentifier::new(arr).unwrap();
    if &oid != arr {
        unreachable!();
    }
    println!("done");
}
