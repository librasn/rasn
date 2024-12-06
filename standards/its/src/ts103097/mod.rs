/// ASN.1 definitions for ETSI TS 103 097 extension module
pub mod extension_module;

use rasn::prelude::*;

pub const ETSI_TS103097_MODULE_OID: &Oid = Oid::const_new(&[0, 4, 0, 5, 5, 103_097, 1, 3, 1]);
