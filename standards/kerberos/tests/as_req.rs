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

    let data: &[u8] = &[
        0x6A, 0x45, 0x30, 0x43,
        0xA1, 0x03, 0x02, 0x01, 0x05,
        0xA2, 0x03, 0x02, 0x01, 0x0A,
        0xA4,
    ];

    let enc = rasn::der::encode(&as_req).unwrap();

    assert_eq!(data, &enc[..data.len()]);
    assert_eq!(as_req, rasn::der::decode(&enc).unwrap());
}
