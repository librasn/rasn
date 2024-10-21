#![doc = include_str!("../README.md")]
#![no_std]

extern crate alloc;

use rasn::prelude::*;

pub const PAL_ZONE: &Oid = Oid::const_new(&[1, 3, 6, 1, 4, 1, 46609, 1, 2]);
pub const PAL_FREQUENCY: &Oid = Oid::const_new(&[1, 3, 6, 1, 4, 1, 46609, 1, 3]);
pub const CBSD_FCCID: &Oid = Oid::const_new(&[1, 3, 6, 1, 4, 1, 46609, 1, 4]);
pub const CBSD_SERIAL: &Oid = Oid::const_new(&[1, 3, 6, 1, 4, 1, 46609, 1, 5]);
pub const SAS_FRN: &Oid = Oid::const_new(&[1, 3, 6, 1, 4, 1, 46609, 1, 6]);
pub const CPIR: &Oid = Oid::const_new(&[1, 3, 6, 1, 4, 1, 46609, 1, 7]);
pub const TEST: &Oid = Oid::const_new(&[1, 3, 6, 1, 4, 1, 46609, 1, 8]);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct CpirId(pub Utf8String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct FccId(pub Utf8String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct Frequency(pub Utf8String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct Frn(pub Utf8String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct Serial(pub Utf8String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct Test(pub Utf8String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate)]
pub struct Zone(pub Utf8String);
