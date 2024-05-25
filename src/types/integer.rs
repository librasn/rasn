use crate::types::{constraints, AsnType, Constraints, Extensible};
use crate::Tag;
use core::ops::{Add, Sub};
use num_bigint::{BigInt, BigUint};
use num_traits::{Num, NumOps, Pow, PrimInt, Signed, ToBytes, ToPrimitive, Zero};

macro_rules! impl_from_integer {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Integer {
                fn from(value: $t) -> Self {
                    #[allow(clippy::cast_possible_truncation)]
                    Integer::Primitive(PrimitiveInteger::<i128>(value as i128))
                }
            }
        )*
    };
}
macro_rules! impl_from_primitive_integer {
    ($($t:ty),*) => {
        $(
            impl From<$t> for PrimitiveInteger<i128> {
fn from(value: $t) -> Self {
                    #[allow(clippy::cast_possible_truncation)]
                    PrimitiveInteger::<i128>(value as i128)
                }
            }
        )*
    };
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryFromIntegerError<T> {
    original: T,
}

impl<T> TryFromIntegerError<T> {
    fn new(original: T) -> Self {
        TryFromIntegerError { original }
    }
    fn __description(&self) -> &str {
        "out of range conversion regarding big integer attempted"
    }
    pub fn into_original(self) -> T {
        self.original
    }
}

impl<T> alloc::fmt::Display for TryFromIntegerError<T> {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        self.__description().fmt(f)
    }
}

macro_rules! impl_try_from_bigint {
    ($T:ty, $to_ty:path) => {
        impl TryFrom<&Integer> for $T {
            type Error = TryFromIntegerError<()>;

            #[inline]
            fn try_from(value: &Integer) -> Result<$T, TryFromIntegerError<()>> {
                match value {
                    Integer::Primitive(PrimitiveInteger::<i128>(value)) => {
                        $to_ty(value).ok_or(TryFromIntegerError::new(()))
                    }
                    Integer::Big(value) => {
                        value.try_into().map_err(|_| TryFromIntegerError::new(()))
                    }
                }
                // $to_ty(value).ok_or(TryFromIntegerError::new(()))
            }
        }

        impl TryFrom<Integer> for $T {
            type Error = TryFromIntegerError<Integer>;

            #[inline]
            fn try_from(value: Integer) -> Result<$T, TryFromIntegerError<Integer>> {
                <$T>::try_from(&value).map_err(|_| TryFromIntegerError::new(value))
            }
        }
    };
}

impl_try_from_bigint!(u8, ToPrimitive::to_u8);
impl_try_from_bigint!(u16, ToPrimitive::to_u16);
impl_try_from_bigint!(u32, ToPrimitive::to_u32);
impl_try_from_bigint!(u64, ToPrimitive::to_u64);
impl_try_from_bigint!(usize, ToPrimitive::to_usize);
// impl_try_from_bigint!(u128, ToPrimitive::to_u128);

impl_try_from_bigint!(i8, ToPrimitive::to_i8);
impl_try_from_bigint!(i16, ToPrimitive::to_i16);
impl_try_from_bigint!(i32, ToPrimitive::to_i32);
impl_try_from_bigint!(i64, ToPrimitive::to_i64);
impl_try_from_bigint!(isize, ToPrimitive::to_isize);
impl_try_from_bigint!(i128, ToPrimitive::to_i128);

// try_into to primitives from Integer
// impl TryFrom<Integer> for i8 {
//     type Error = crate::error::DecodeError;
//     fn try_from(value: Integer) -> Result<Sef, Self::Error> {
//         match value {
//             Integer::Primitive(PrimitiveInteger::<i128>(value)) => {
//                 if value > i8::MAX as i128 || value < i8::MIN as i128 {
//                     Err(crate::Error::InvalidInteger)
//                 } else {
//                     Ok(value as i8)
//                 }
//             }
//             Integer::Big(value) => {
//                 if value > i8::MAX as i128 || value < i8::MIN as i128 {
//                     Err(crate::Error::InvalidInteger)
//                 } else {
//                     Ok(value.to_i8().unwrap())
//                 }
//             }
//         }
//     }
// }
//
impl From<BigInt> for Integer {
    fn from(value: BigInt) -> Self {
        Integer::Big(value)
    }
}
impl From<u128> for Integer {
    fn from(value: u128) -> Self {
        Integer::Big(value.into())
    }
}

// #[cfg(target_pointer_width = "64")]
impl_from_primitive_integer! {
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    usize
}
impl_from_integer! {
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    u8,
    u16,
    u32,
    u64,
    usize
}
/// Primitive integers can bring significant performance improvements
/// As a result, native word size and i128/u128 are supported as smaller integers
/// Note: all architectures do not support i128/u128
/// 128-support
#[derive(Debug, Clone, Copy, Ord, Hash, Eq, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct PrimitiveInteger<T: PrimInt + Num + NumOps + Signed>(T);

const PRIMITIVE_BYTE_WIDTH: usize = core::mem::size_of::<i128>();

impl<T: PrimInt + Num + NumOps + Signed + ToBytes> core::ops::Deref for PrimitiveInteger<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T: PrimInt + Num + NumOps + Signed + ToBytes> PrimitiveInteger<T> {
    #[must_use]
    pub const fn value(&self) -> T {
        self.0
    }
    #[inline]
    /// Optimized primitive integer conversion to bytes
    pub fn unsafe_minimal_primitive_ne_bytes(&self, signed: bool) -> &[u8] {
        let ptr = &self.0 as *const T as *const u8; // Cast to a pointer to the first byte
        let bytes_needed = if signed {
            self.signed_bytes_needed()
        } else {
            self.unsigned_bytes_needed()
        };
        unsafe { core::slice::from_raw_parts(ptr, bytes_needed) }
    }
    fn unsigned_bytes_needed(&self) -> usize {
        if self.0.is_zero() {
            1
        } else {
            PRIMITIVE_BYTE_WIDTH - (self.0.leading_zeros() / 8) as usize
        }
    }

    fn signed_bytes_needed(&self) -> usize {
        if self.0.is_negative() {
            let leading_ones = self.0.leading_ones() as usize;
            if leading_ones % 8 == 0 {
                PRIMITIVE_BYTE_WIDTH - leading_ones / 8 + 1
            } else {
                PRIMITIVE_BYTE_WIDTH - leading_ones / 8
            }
        } else {
            let leading_zeroes = self.0.leading_zeros() as usize;
            if leading_zeroes % 8 == 0 {
                PRIMITIVE_BYTE_WIDTH - leading_zeroes / 8 + 1
            } else {
                PRIMITIVE_BYTE_WIDTH - leading_zeroes / 8
            }
        }
    }

    fn primitive_int_le_bytes(&self) -> [u8; PRIMITIVE_BYTE_WIDTH] {
        #[cfg(target_endian = "big")]
        {
            self.0
                .to_le_bytes()
                .as_ref()
                .try_into()
                .expect("Invalid byte width! Should not happen.")
        }
        #[cfg(target_endian = "little")]
        {
            self.0
                .to_ne_bytes()
                .as_ref()
                .try_into()
                .expect("Invalid byte width! Should not happen.")
        }
    }
    pub fn primitive_needed_bytes(&self, signed: bool) -> ([u8; PRIMITIVE_BYTE_WIDTH], usize) {
        let bytes: [u8; PRIMITIVE_BYTE_WIDTH] = self.primitive_int_le_bytes();
        let needed = if signed {
            self.signed_bytes_needed()
        } else {
            self.unsigned_bytes_needed()
        };
        let mut slice_reversed: [u8; 16] = [0; 16];
        let len = needed;
        for i in 0..len {
            slice_reversed[i] = bytes[len - 1 - i];
        }
        (slice_reversed, needed)
    }
}

//
#[derive(Debug, Clone, Ord, Hash, Eq, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Integer {
    Primitive(PrimitiveInteger<i128>),
    Big(BigInt),
}
impl Integer {
    /// encoding and decoding functions, and use fixed-size arrays to get byte presentation
    #[must_use]
    pub fn as_primitive(&self) -> Option<&PrimitiveInteger<i128>> {
        match self {
            Integer::Primitive(ref inner) => Some(inner),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_i128(&self) -> Option<i128> {
        match self {
            Integer::Primitive(value) => Some(**value),
            _ => None,
        }
    }
    #[must_use]
    pub fn as_big(&self) -> Option<&BigInt> {
        match self {
            Integer::Big(ref inner) => Some(inner),
            _ => None,
        }
    }
    /// Will always create new heap allocation, use carefully
    #[must_use]
    #[inline]
    pub fn to_be_bytes(&self) -> alloc::vec::Vec<u8> {
        match self {
            Integer::Primitive(value) => value.to_be_bytes().to_vec(),
            Integer::Big(value) => value.to_signed_bytes_be(),
            // Integer::Big(value) => value.to_biguint().unwrap().to_bytes_be(),
        }
    }

    #[must_use]
    #[inline]
    pub fn to_unsigned_be_bytes(&self) -> Option<alloc::vec::Vec<u8>> {
        match self {
            Integer::Primitive(value) => {
                let value = value.unsafe_minimal_primitive_ne_bytes(false);
                Some(value.to_vec())
            }
            Integer::Big(value) => Some(value.to_biguint()?.to_bytes_be()),
        }
    }

    #[must_use]
    pub fn from_signed_be_bytes(bytes: &[u8]) -> Self {
        if bytes.len() <= 16 {
            if bytes[0] < 0b1000_0000 {
                let mut array = [0u8; 16];
                array[16 - bytes.len()..].copy_from_slice(bytes);
                let value = i128::from_be_bytes(array);
                Integer::Primitive(PrimitiveInteger::<i128>(value))
            } else {
                let mut array = [0xffu8; 16];
                dbg!(bytes);
                dbg!(bytes.len());
                array[16 - bytes.len()..].copy_from_slice(bytes);
                dbg!(&array);
                Integer::Primitive(PrimitiveInteger::<i128>(i128::from_be_bytes(array)))
            }
        } else {
            Integer::Big(BigInt::from_signed_bytes_be(bytes))
        }
    }
    #[must_use]
    pub fn from_be_bytes(bytes: &[u8]) -> Self {
        // For u128 to be able to fit into i128, the byte length must be less or equal than 15 bytes
        if bytes.len() <= 16 {
            let mut array = [0u8; 16];
            for i in 0..bytes.len() {
                array[16 - bytes.len() + i] = bytes[i];
            }
            Integer::Primitive(PrimitiveInteger::<i128>(i128::from_be_bytes(array)))
        } else {
            Integer::Big(BigUint::from_bytes_be(bytes).into())
        }
    }
    #[must_use]
    pub fn bits(&self) -> u64 {
        // TODO: will overflow and panic on very large numbers
        match self {
            Integer::Primitive(value) => value.to_be_bytes().len() as u64 * 8u64,
            Integer::Big(value) => value.bits(),
        }
    }
    #[must_use]
    pub fn signum(&self) -> Self {
        match self {
            Integer::Primitive(value) => {
                Integer::Primitive(PrimitiveInteger::<i128>(value.signum()))
            }
            Integer::Big(value) => Integer::Big(value.signum()),
        }
    }
    #[must_use]
    pub fn is_positive(&self) -> bool {
        match self {
            Integer::Primitive(value) => value.is_positive(),
            Integer::Big(value) => value.is_positive(),
        }
    }
    #[must_use]
    pub fn is_negative(&self) -> bool {
        match self {
            Integer::Primitive(value) => value.is_negative(),
            Integer::Big(value) => value.is_negative(),
        }
    }
    #[must_use]
    pub fn is_zero(&self) -> bool {
        match self {
            Integer::Primitive(value) => value.is_zero(),
            Integer::Big(value) => value.is_zero(),
        }
    }
}

impl Sub for Integer {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Integer::Primitive(lhs), Integer::Primitive(rhs)) => {
                Integer::Big(BigInt::from(*lhs) - *rhs)
            }
            (Integer::Big(lhs), Integer::Big(rhs)) => Integer::Big(lhs - rhs),
            (Integer::Primitive(lhs), Integer::Big(rhs)) => Integer::Big(*lhs - rhs),
            (Integer::Big(lhs), Integer::Primitive(rhs)) => Integer::Big(lhs - *rhs),
        }
    }
}
impl Add for Integer {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Integer::Primitive(lhs), Integer::Primitive(rhs)) => {
                Integer::Big(BigInt::from(*lhs) + *rhs)
            }
            (Integer::Big(lhs), Integer::Big(rhs)) => Integer::Big(lhs + rhs),
            (Integer::Primitive(lhs), Integer::Big(rhs)) => Integer::Big(*lhs + rhs),
            (Integer::Big(lhs), Integer::Primitive(rhs)) => Integer::Big(lhs + *rhs),
        }
    }
}

impl Pow<Integer> for Integer {
    type Output = Self;

    /// Will silently truncate too large exponents
    fn pow(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Integer::Primitive(lhs), Integer::Primitive(rhs)) => {
                Integer::Big(BigInt::from(*lhs).pow(rhs.to_u32().unwrap()))
            }
            (Integer::Big(lhs), Integer::Big(rhs)) => Integer::Big(lhs.pow(rhs.to_u32().unwrap())),
            (Integer::Primitive(lhs), Integer::Big(rhs)) => {
                Integer::Big(BigInt::from(*lhs).pow(rhs.to_u32().unwrap()))
            }
            (Integer::Big(lhs), Integer::Primitive(rhs)) => Integer::Big(lhs.pow(*rhs as u128)),
        }
    }
}

impl core::ops::Shl<Integer> for Integer {
    type Output = Self;

    fn shl(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Integer::Primitive(lhs), Integer::Primitive(rhs)) => {
                Integer::Primitive(PrimitiveInteger::<i128>(*lhs << *rhs))
            }
            (Integer::Big(lhs), Integer::Big(rhs)) => Integer::Big(lhs << rhs.to_i128().unwrap()),
            (Integer::Primitive(lhs), Integer::Big(rhs)) => {
                Integer::Big((*lhs << rhs.to_i128().unwrap()).into())
            }
            (Integer::Big(lhs), Integer::Primitive(rhs)) => Integer::Big(lhs << *rhs),
        }
    }
}

impl alloc::fmt::Display for Integer {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        match self {
            Integer::Primitive(value) => write!(f, "{}", **value),
            Integer::Big(value) => write!(f, "{value}"),
        }
    }
}

impl core::default::Default for Integer {
    fn default() -> Self {
        Integer::Primitive(PrimitiveInteger::<i128>(0))
    }
}

///
/// A integer which has encoded constraint range between `START` and `END`.
#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ConstrainedInteger<const START: i128, const END: i128>(pub(crate) Integer);

impl<const START: i128, const END: i128> AsnType for ConstrainedInteger<START, END> {
    const TAG: Tag = Tag::INTEGER;
    const CONSTRAINTS: Constraints<'static> =
        Constraints::new(&[constraints::Constraint::Value(Extensible::new(
            constraints::Value::new(constraints::Bounded::const_new(START, END)),
        ))]);
}

impl<const START: i128, const END: i128> core::ops::Deref for ConstrainedInteger<START, END> {
    type Target = Integer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Into<Integer>, const START: i128, const END: i128> From<T>
    for ConstrainedInteger<START, END>
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
