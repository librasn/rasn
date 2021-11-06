use rasn::prelude::*;

use rasn_pkix::AlgorithmIdentifier;

use super::{EncryptedData, KerberosFlags, KerberosString, KerberosTime, LastReq};

// reserved(0),
// systemSetPin(1),
// mandatory(2)
pub type PinFlags = KerberosFlags;
pub type AnyUri = Utf8String;

#[derive(AsnType, Decode, Encode)]
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

#[derive(AsnType, Decode, Encode)]
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

// decimal(0),
// hexadecimal(1),
// alphanumeric(2),
// binary(3),
// base64(4)
pub type OtpFormat = Integer;

// reserved(0),
// nextOTP(1),
// combine(2),
// collect-pin(3),
// do-not-collect-pin(4),
// must-encrypt-nonce (5),
// separate-pin-required (6),
// check-digit (7)
pub type OtpFlags = KerberosFlags;

#[derive(AsnType, Decode, Encode)]
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

#[derive(AsnType, Decode, Encode)]
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

#[derive(AsnType, Decode, Encode)]
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
