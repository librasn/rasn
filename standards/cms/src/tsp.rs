//! [RFC 3161](https://www.rfc-editor.org/rfc/rfc3161) Time Stamp Protocol (TSP).

use crate::ContentInfo;
use rasn::prelude::*;
use rasn::types::OctetString;
use rasn::{AsnType, Decode, Encode};
use rasn_pkix::{AlgorithmIdentifier, Extensions, GeneralName};

/// Time-stamp token eContentType of EncapsulatedContentInfo in the SignedData.
pub const TST_INFO: &Oid = Oid::ISO_MEMBER_BODY_US_RSADSI_PKCS9_SMIME_CT_TSTINFO;

fn default_false() -> bool {
    false
}

/** Time-stamp request.

[RFC 3161 2.4.1](https://www.rfc-editor.org/rfc/rfc3161#section-2.4.1):

```text
   TimeStampReq ::= SEQUENCE  {
      version                      INTEGER  { v1(1) },
      messageImprint               MessageImprint,
        --a hash algorithm OID and the hash value of the data to be
        --time-stamped
      reqPolicy             TSAPolicyId              OPTIONAL,
      nonce                 INTEGER                  OPTIONAL,
      certReq               BOOLEAN                  DEFAULT FALSE,
      extensions            [0] IMPLICIT Extensions  OPTIONAL  }
```
*/
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeStampReq {
    pub version: u64,
    pub message_imprint: MessageImprint,
    pub req_policy: Option<TsaPolicyId>,
    pub nonce: Option<Integer>,
    #[rasn(default = "default_false")]
    pub cert_req: bool,
    #[rasn(tag(0))]
    pub extensions: Option<Extensions>,
}

/** Fingerprint of data to protect with a time-stamp.

[RFC 3161 2.4.1](https://www.rfc-editor.org/rfc/rfc3161#section-2.4.1):

```text
   MessageImprint ::= SEQUENCE  {
           hashAlgorithm                AlgorithmIdentifier,
           hashedMessage                OCTET STRING  }
```
*/
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageImprint {
    pub hash_algorithm: AlgorithmIdentifier,
    pub hashed_message: OctetString,
}

/** A policy that applies to a time-stamp.

[RFC 3161 2.4.1](https://www.rfc-editor.org/rfc/rfc3161#section-2.4.1):

```text
   TSAPolicyId ::= OBJECT IDENTIFIER
```
*/
pub type TsaPolicyId = ObjectIdentifier;

/** Time-stamp response.

[RFC 3161 2.4.2](https://www.rfc-editor.org/rfc/rfc3161#section-2.4.2):

```text
   TimeStampResp ::= SEQUENCE  {
      status                  PKIStatusInfo,
      timeStampToken          TimeStampToken     OPTIONAL  }
```
*/
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TimeStampResp {
    pub status: PkiStatusInfo,
    pub time_stamp_token: Option<TimeStampToken>,
}

/** Time-stamp response status.

[RFC 3161 2.4.2](https://www.rfc-editor.org/rfc/rfc3161#section-2.4.2):

```text
   PKIStatusInfo ::= SEQUENCE {
      status        PKIStatus,
      statusString  PKIFreeText     OPTIONAL,
      failInfo      PKIFailureInfo  OPTIONAL  }
```
*/
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PkiStatusInfo {
    pub status: PkiStatus,
    pub status_string: Option<PkiFreeText>,
    pub fail_info: Option<PkiFailureInfo>,
}

/** Time-stamp response status code.

[RFC 3161 2.4.2](https://www.rfc-editor.org/rfc/rfc3161#section-2.4.2):

```text
   PKIStatus ::= INTEGER {
      granted                (0),
      -- when the PKIStatus contains the value zero a TimeStampToken, as
         requested, is present.
      grantedWithMods        (1),
       -- when the PKIStatus contains the value one a TimeStampToken,
         with modifications, is present.
      rejection              (2),
      waiting                (3),
      revocationWarning      (4),
       -- this message contains a warning that a revocation is
       -- imminent
      revocationNotification (5)
       -- notification that a revocation has occurred  }
```
*/
#[derive(AsnType, Clone, Copy, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[rasn(enumerated)]
pub enum PkiStatus {
    /// A TimeStampToken, as requested, is present.
    Granted = 0,
    /// A TimeStampToken, with modifications, is present.
    GrantedWithMods = 1,
    Rejection = 2,
    Waiting = 3,
    /// This message contains a warning that a revocation is imminent.
    RevocationWarning = 4,
    /// Notification that a revocation has occurred.
    RevocationNotification = 5,
}

/** Time-stamp response status free text.

[RFC 3161 2.4.2](https://www.rfc-editor.org/rfc/rfc3161#section-2.4.2):

```text
   PKIFreeText ::= SEQUENCE SIZE (1..MAX) OF UTF8String
       -- text encoded as UTF-8 String (note:  each UTF8String SHOULD
       -- include an RFC 1766 language tag to indicate the language
       -- of the contained text)
```
*/
pub type PkiFreeText = SequenceOf<Utf8String>;

/** Time-stamp response status failure reason.

[RFC 3161 2.4.2](https://www.rfc-editor.org/rfc/rfc3161#section-2.4.2):

```text
   PKIFailureInfo ::= BIT STRING {
       badAlg               (0),
         -- unrecognized or unsupported Algorithm Identifier
       badRequest           (2),
         -- transaction not permitted or supported
       badDataFormat        (5),
         -- the data submitted has the wrong format
       timeNotAvailable    (14),
         -- the TSA's time source is not available
       unacceptedPolicy    (15),
         -- the requested TSA policy is not supported by the TSA
       unacceptedExtension (16),
         -- the requested extension is not supported by the TSA
       addInfoNotAvailable (17)
         -- the additional information requested could not be understood
         -- or is not available
       systemFailure       (25)
         -- the request cannot be handled due to system failure  }
```
*/
pub type PkiFailureInfo = BitString;

/** Time-stamp token.

[RFC 3161 2.4.2](https://www.rfc-editor.org/rfc/rfc3161#section-2.4.2):

```text
   TimeStampToken ::= ContentInfo
     -- contentType is id-signedData ([CMS])
     -- content is SignedData ([CMS])
```
*/
pub type TimeStampToken = ContentInfo;

/** Time-stamp token information.

[RFC 3161 2.4.2](https://www.rfc-editor.org/rfc/rfc3161#section-2.4.2):

```text
   TSTInfo ::= SEQUENCE  {
      version                      INTEGER  { v1(1) },
      policy                       TSAPolicyId,
      messageImprint               MessageImprint,
        -- MUST have the same value as the similar field in
        -- TimeStampReq
      serialNumber                 INTEGER,
       -- Time-Stamping users MUST be ready to accommodate integers
       -- up to 160 bits.
      genTime                      GeneralizedTime,
      accuracy                     Accuracy                 OPTIONAL,
      ordering                     BOOLEAN             DEFAULT FALSE,
      nonce                        INTEGER                  OPTIONAL,
        -- MUST be present if the similar field was present
        -- in TimeStampReq.  In that case it MUST have the same value.
      tsa                          [0] GeneralName          OPTIONAL,
      extensions                   [1] IMPLICIT Extensions   OPTIONAL  }
```
*/
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TstInfo {
    pub version: Integer,
    pub policy: TsaPolicyId,
    pub message_imprint: MessageImprint,
    pub serial_number: Integer,
    pub gen_time: GeneralizedTime,
    pub accuracy: Option<Accuracy>,
    #[rasn(default = "default_false")]
    pub ordering: bool,
    pub nonce: Option<Integer>,
    #[rasn(tag(explicit(0)))]
    pub tsa: Option<GeneralName>,
    #[rasn(tag(1))]
    pub extensions: Option<Extensions>,
}

/** Accuracy of the time (`genTime`) in [TstInfo].

[RFC 3161 2.4.2](https://www.rfc-editor.org/rfc/rfc3161#section-2.4.2):

```text
   Accuracy ::= SEQUENCE {
         seconds        INTEGER              OPTIONAL,
         millis     [0] INTEGER  (1..999)    OPTIONAL,
         micros     [1] INTEGER  (1..999)    OPTIONAL  }
```
*/
#[derive(AsnType, Clone, Debug, Decode, Encode, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Accuracy {
    pub seconds: Option<Integer>,
    #[rasn(tag(0))]
    pub millis: Option<Integer>,
    #[rasn(tag(1))]
    pub micros: Option<Integer>,
}
