#![doc = include_str!("../README.md")]
#![no_std]

use rasn::prelude::*;

/// Anonymous SEQUENCE OF member
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(identifier = "SEQUENCE")]
pub struct AnonymousNegoData {
    #[rasn(tag(context, 0), identifier = "negoToken")]
    pub nego_token: OctetString,
}

impl AnonymousNegoData {
    pub fn new(nego_token: OctetString) -> Self {
        Self { nego_token }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct NegoData(pub SequenceOf<AnonymousNegoData>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TSCredentials {
    #[rasn(tag(explicit(context, 0)), identifier = "credType")]
    pub cred_type: Integer,
    #[rasn(tag(explicit(context, 1)))]
    pub credentials: OctetString,
}

impl TSCredentials {
    pub fn new(cred_type: Integer, credentials: OctetString) -> Self {
        Self {
            cred_type,
            credentials,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TSCspDataDetail {
    #[rasn(tag(explicit(context, 0)), identifier = "keySpec")]
    pub key_spec: Integer,
    #[rasn(tag(explicit(context, 1)), identifier = "cardName")]
    pub card_name: Option<OctetString>,
    #[rasn(tag(explicit(context, 2)), identifier = "readerName")]
    pub reader_name: Option<OctetString>,
    #[rasn(tag(explicit(context, 3)), identifier = "containerName")]
    pub container_name: Option<OctetString>,
    #[rasn(tag(explicit(context, 4)), identifier = "cspName")]
    pub csp_name: Option<OctetString>,
}

impl TSCspDataDetail {
    pub fn new(
        key_spec: Integer,
        card_name: Option<OctetString>,
        reader_name: Option<OctetString>,
        container_name: Option<OctetString>,
        csp_name: Option<OctetString>,
    ) -> Self {
        Self {
            key_spec,
            card_name,
            reader_name,
            container_name,
            csp_name,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TSPasswordCreds {
    #[rasn(tag(explicit(context, 0)), identifier = "domainName")]
    pub domain_name: OctetString,
    #[rasn(tag(explicit(context, 1)), identifier = "userName")]
    pub user_name: OctetString,
    #[rasn(tag(explicit(context, 2)))]
    pub password: OctetString,
}

impl TSPasswordCreds {
    pub fn new(domain_name: OctetString, user_name: OctetString, password: OctetString) -> Self {
        Self {
            domain_name,
            user_name,
            password,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TSRemoteGuardCreds {
    #[rasn(tag(explicit(context, 0)), identifier = "logonCred")]
    pub logon_cred: TSRemoteGuardPackageCred,
    #[rasn(tag(explicit(context, 1)), identifier = "supplementalCreds")]
    pub supplemental_creds: Option<SequenceOf<TSRemoteGuardPackageCred>>,
}

impl TSRemoteGuardCreds {
    pub fn new(
        logon_cred: TSRemoteGuardPackageCred,
        supplemental_creds: Option<SequenceOf<TSRemoteGuardPackageCred>>,
    ) -> Self {
        Self {
            logon_cred,
            supplemental_creds,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TSRemoteGuardPackageCred {
    #[rasn(tag(explicit(context, 0)), identifier = "packageName")]
    pub package_name: OctetString,
    #[rasn(tag(explicit(context, 1)), identifier = "credBuffer")]
    pub cred_buffer: OctetString,
}

impl TSRemoteGuardPackageCred {
    pub fn new(package_name: OctetString, cred_buffer: OctetString) -> Self {
        Self {
            package_name,
            cred_buffer,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TSRequest {
    #[rasn(tag(explicit(context, 0)))]
    pub version: Integer,
    #[rasn(tag(explicit(context, 1)), identifier = "negoTokens")]
    pub nego_tokens: Option<NegoData>,
    #[rasn(tag(explicit(context, 2)), identifier = "authInfo")]
    pub auth_info: Option<OctetString>,
    #[rasn(tag(explicit(context, 3)), identifier = "pubKeyAuth")]
    pub pub_key_auth: Option<OctetString>,
    #[rasn(tag(explicit(context, 4)), identifier = "errorCode")]
    pub error_code: Option<Integer>,
    #[rasn(tag(explicit(context, 5)), identifier = "clientNonce")]
    pub client_nonce: Option<OctetString>,
}

impl TSRequest {
    pub fn new(
        version: Integer,
        nego_tokens: Option<NegoData>,
        auth_info: Option<OctetString>,
        pub_key_auth: Option<OctetString>,
        error_code: Option<Integer>,
        client_nonce: Option<OctetString>,
    ) -> Self {
        Self {
            version,
            nego_tokens,
            auth_info,
            pub_key_auth,
            error_code,
            client_nonce,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
pub struct TSSmartCardCreds {
    #[rasn(tag(explicit(context, 0)))]
    pub pin: OctetString,
    #[rasn(tag(explicit(context, 1)), identifier = "cspData")]
    pub csp_data: TSCspDataDetail,
    #[rasn(tag(explicit(context, 2)), identifier = "userHint")]
    pub user_hint: Option<OctetString>,
    #[rasn(tag(explicit(context, 3)), identifier = "domainHint")]
    pub domain_hint: Option<OctetString>,
}

impl TSSmartCardCreds {
    pub fn new(
        pin: OctetString,
        csp_data: TSCspDataDetail,
        user_hint: Option<OctetString>,
        domain_hint: Option<OctetString>,
    ) -> Self {
        Self {
            pin,
            csp_data,
            user_hint,
            domain_hint,
        }
    }
}
