use rasn::{de::Decode, types::*, *};

#[derive(Debug, Clone, AsnType, Decode, Encode)]
#[rasn(choice)]
pub enum GeneralName {
    //Rfc822Name(Ia5String),
    #[rasn(tag(context, 2))]
    DnsName(Ia5String),
    //DirectoryName(Name),
    //EdiPartyName(EdiPartyName),
    #[rasn(tag(context, 6))]
    Uri(Ia5String),
    //IpAddress(OctetString),
    //RegisteredId(Oid),
}

#[derive(Debug, AsnType, Decode, Encode)]
#[rasn(delegate)]
pub struct GeneralNames(pub Vec<GeneralName>);

#[derive(Debug, AsnType, Decode, Encode)]
#[rasn(delegate)]
pub(crate) struct DistributionPoints(pub(crate) Vec<DistributionPoint>);

#[derive(Debug, AsnType, Decode, Encode)]
pub(crate) struct DistributionPoint {
    #[rasn(tag(context, 0))]
    #[rasn(choice)]
    pub(crate) d_point: Option<DistributionPointName>,
    //pub(crate) d_point: DistributionPointName,

    //pub(crate) reasons: Option<BitString>,
    //#[rasn(tag(context, 2))]
    //pub(crate) crl_issuer: Option<GeneralNames>,
}

#[derive(Debug, AsnType, Decode, Encode)]
#[rasn(choice)]
pub(crate) enum DistributionPointName {
    #[rasn(tag(context, 0))]
    FullName(Vec<GeneralName>),
    //Relative(RelativeDistinguishedName),
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};

    #[rustfmt::skip]
    fn encoded() -> Vec<u8> {
        [
        0x30, 0x78, 0x30, 0x3a, 0xa0, 0x38, 0xa0, 0x36, 0x86, 0x34, 0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, // 0x0:.8.6.4http:/
        0x2f, 0x63, 0x72, 0x6c, 0x33, 0x2e, 0x64, 0x69, 0x67, 0x69, 0x63, 0x65, 0x72, 0x74, 0x2e, 0x63, // /crl3.digicert.c
        0x6f, 0x6d, 0x2f, 0x44, 0x69, 0x67, 0x69, 0x43, 0x65, 0x72, 0x74, 0x41, 0x73, 0x73, 0x75, 0x72, // om/DigiCertAssur
        0x65, 0x64, 0x49, 0x44, 0x52, 0x6f, 0x6f, 0x74, 0x43, 0x41, 0x2e, 0x63, 0x72, 0x6c, 0x30, 0x3a, // edIDRootCA.crl0:
        0xa0, 0x38, 0xa0, 0x36, 0x86, 0x34, 0x68, 0x74, 0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x63, 0x72, 0x6c, // .8.6.4http://crl
        0x34, 0x2e, 0x64, 0x69, 0x67, 0x69, 0x63, 0x65, 0x72, 0x74, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x44, // 4.digicert.com/D
        0x69, 0x67, 0x69, 0x43, 0x65, 0x72, 0x74, 0x41, 0x73, 0x73, 0x75, 0x72, 0x65, 0x64, 0x49, 0x44, // igiCertAssuredID
        0x52, 0x6f, 0x6f, 0x74, 0x43, 0x41, 0x2e, 0x63, 0x72, 0x6c,                                     // RootCA.crl
        ].into()
    }

    #[test]
    fn test_general_name() {
        let encoded = encoded();
        let g: DistributionPoints = rasn::der::decode(&encoded).unwrap();
    }

    #[test]
    fn test_encode_dp() {
        let name1: Utf8String = "http://crl3.digicert.com/DigiCertAssuredIDRootCA.crl".into();
        let name1 = GeneralName::Uri(name1.into());

        let name2: Utf8String = "http://crl4.digicert.com/DigiCertAssuredIDRootCA.crl".into();
        let name2 = GeneralName::Uri(name2.into());

        let name = DistributionPointName::FullName(vec![name1, name2]);
        let dp = DistributionPoint {
            d_point: name.into(),
        };
        let dps = vec![dp];
        let enc = rasn::der::encode(&dps).unwrap();
        let expected = encoded();
        // assert_eq!(expected, enc);
    }
}
