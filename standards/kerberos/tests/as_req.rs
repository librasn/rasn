use rasn::prelude::*;
use rasn_kerberos::*;

use pretty_assertions::assert_eq;

const _: () = assert!(AsReq::TAG.const_eq(&Tag::new(Class::Application, 10)));

#[test]
fn as_req() {
    let as_req = AsReq(KdcReq {
        pvno: Integer::parse_bytes(b"5", 10).unwrap(),
        msg_type: Integer::parse_bytes(b"10", 10).unwrap(),
        padata: None,
        req_body: KdcReqBody {
            kdc_options: KdcOptions(KerberosFlags::from_vec(vec!(0;0))),
            cname: None,
            realm: KerberosString::new("COMPANY.INT".to_string()),
            sname: None,
            from: None,
            till: KerberosTime(GeneralizedTime::parse_from_rfc2822("Fri, 04 Feb 2022 10:11:11 GMT").unwrap()),
            rtime: None,
            nonce: 123514514,
            etype: vec![18],
            addresses: None,
            enc_authorization_data: None,
            additional_tickets: None,
        }
    });

    let enc = rasn::der::encode(&as_req).unwrap();

    assert_eq!(as_req, rasn::der::decode(&enc).unwrap());
    println!("{:02X?}", enc);
    assert!(enc.starts_with(&[0x6A]));
}
