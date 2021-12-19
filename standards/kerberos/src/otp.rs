use rasn::prelude::*;

use rasn_pkix::AlgorithmIdentifier;

use super::{EncryptedData, KerberosFlags, KerberosString, KerberosTime, LastReq};

pub type AnyUri = Utf8String;
pub type AdAuthenticationIndicator = SequenceOf<Utf8String>;

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct PinFlags(pub KerberosFlags);

impl PinFlags {
    pub fn reserved() -> Self {
        Self(KerberosFlags::from_element(0))
    }

    pub fn system_set_pin() -> Self {
        Self(KerberosFlags::from_element(1))
    }

    pub fn mandatory() -> Self {
        Self(KerberosFlags::from_element(2))
    }
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct PaOtpChallenge {
    #[rasn(tag(0))]
    pub nonce: OctetString,
    #[rasn(tag(1))]
    pub otp_service: Option<Utf8String>,
    #[rasn(tag(2))]
    pub otp_token_info: SequenceOf<OtpTokenInfo>,
    #[rasn(tag(3))]
    pub salt: Option<KerberosString>,
    #[rasn(tag(4))]
    pub s2kparams: Option<OctetString>,
}

impl PaOtpChallenge {
    pub fn new(nonce: OctetString, otp_token_info: SequenceOf<OtpTokenInfo>) -> Self {
        Self {
            nonce,
            otp_service: None,
            otp_token_info,
            salt: None,
            s2kparams: None,
        }
    }
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct OtpTokenInfo {
    #[rasn(tag(0))]
    pub flags: OtpFlags,
    #[rasn(tag(1))]
    pub otp_vendor: Option<Utf8String>,
    #[rasn(tag(2))]
    pub otp_challenge: Option<OctetString>,
    #[rasn(tag(3))]
    pub otp_length: Option<i32>,
    #[rasn(tag(4))]
    pub otp_format: Option<OtpFormat>,
    #[rasn(tag(5))]
    pub otp_token_id: Option<OctetString>,
    #[rasn(tag(6))]
    pub otp_alg_id: Option<AnyUri>,
    #[rasn(tag(7))]
    pub supported_hash_alg: Option<SequenceOf<AlgorithmIdentifier>>,
    #[rasn(tag(8))]
    pub iteration_count: Option<i32>,
}

impl OtpTokenInfo {
    pub fn new(flags: OtpFlags) -> Self {
        Self {
            flags,
            otp_vendor: None,
            otp_challenge: None,
            otp_length: None,
            otp_format: None,
            otp_token_id: None,
            otp_alg_id: None,
            supported_hash_alg: None,
            iteration_count: None,
        }
    }
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct OtpFormat(pub Integer);

impl OtpFormat {
    pub fn decimal() -> Self {
        Self(0.into())
    }

    pub fn hexadecimal() -> Self {
        Self(1.into())
    }

    pub fn alphanumeric() -> Self {
        Self(2.into())
    }

    pub fn binary() -> Self {
        Self(3.into())
    }

    pub fn base64() -> Self {
        Self(4.into())
    }
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(delegate)]
pub struct OtpFlags(pub KerberosFlags);

impl OtpFlags {
    pub fn reserved() -> Self {
        Self(0.into())
    }

    pub fn nextOTP() -> Self {
        Self(1.into())
    }

    pub fn combine() -> Self {
        Self(2.into())
    }

    pub fn collect_pin() -> Self {
        Self(3.into())
    }

    pub fn do_not_collect_pin() -> Self {
        Self(4.into())
    }

    pub fn must_encrypt_nonce() -> Self {
        Self(5.into())
    }

    pub fn separate_pin_required() -> Self {
        Self(6.into())
    }

    pub fn check_digit() -> Self {
        Self(7.into())
    }
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct PaOtpRequest {
    #[rasn(tag(0))]
    pub flags: OtpFlags,
    #[rasn(tag(1))]
    pub nonce: Option<OctetString>,
    #[rasn(tag(2))]
    pub enc_data: EncryptedData,
    #[rasn(tag(3))]
    pub hash_alg: Option<AlgorithmIdentifier>,
    #[rasn(tag(4))]
    pub iteration_count: Option<i32>,
    #[rasn(tag(5))]
    pub otp_value: Option<OctetString>,
    #[rasn(tag(6))]
    pub otp_pin: Option<Utf8String>,
    #[rasn(tag(7))]
    pub otp_challenge: Option<OctetString>,
    #[rasn(tag(8))]
    pub otp_time: Option<KerberosTime>,
    #[rasn(tag(9))]
    pub otp_counter: Option<OctetString>,
    #[rasn(tag(10))]
    pub otp_format: Option<OtpFormat>,
    #[rasn(tag(11))]
    pub otp_token_id: Option<OctetString>,
    #[rasn(tag(12))]
    pub otp_alg_id: Option<AnyUri>,
    #[rasn(tag(13))]
    pub otp_vendor: Option<Utf8String>,
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct PaOtpEncRequest {
    #[rasn(tag(0))]
    pub nonce: OctetString,
}

impl PaOtpEncRequest {
    pub fn new(nonce: OctetString) -> Self {
        Self { nonce }
    }
}

#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct PaOtpPinChange {
    #[rasn(tag(0))]
    pub flags: PinFlags,
    #[rasn(tag(1))]
    pub pin: Option<Utf8String>,
    #[rasn(tag(2))]
    pub min_length: Option<Integer>,
    #[rasn(tag(3))]
    pub max_length: Option<Integer>,
    #[rasn(tag(4))]
    pub last_req: Option<LastReq>,
    #[rasn(tag(5))]
    pub format: Option<OtpFormat>,
}

impl PaOtpPinChange {
    pub fn new(flags: PinFlags) -> Self {
        Self {
            flags,
            pin: None,
            min_length: None,
            max_length: None,
            last_req: None,
            format: None,
        }
    }
}
