use rasn::{
    ber,
    types::{Class, Constraints, Identifier, Tag},
    Decoder, Encoder,
};

use rasn_kerberos::KerberosFlags;

#[test]
fn kerberos_flags_dec() {
    let input = b"\x03\x05\x00\x40\x81\x00\x00";
    let mut decoder = ber::de::Decoder::new(input, ber::de::DecoderOptions::ber());
    let output = decoder
        .decode_bit_string(Tag::new(Class::Universal, 3), Constraints::default())
        .unwrap();
    let expected = KerberosFlags::from_vec([0x40, 0x81, 0x00, 0x00].to_vec());
    assert_eq!(output, expected)
}

#[test]
fn kerberos_flags_enc() {
    let bitstring = KerberosFlags::from_vec([0x40, 0x81, 0x00, 0x00].to_vec());
    let mut encoder = ber::enc::Encoder::new(ber::enc::EncoderOptions::ber());
    encoder
        .encode_bit_string(
            Tag::new(Class::Universal, 3),
            Constraints::default(),
            &bitstring,
            Identifier::EMPTY,
        )
        .unwrap();
    assert_eq!(
        encoder.output(),
        vec![0x03, 0x05, 0x00, 0x40, 0x81, 0x00, 0x00]
    )
}
