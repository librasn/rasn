use crate::types::{AsnType, Constraints, Extensible, Tag, constraints};
use alloc::boxed::Box;
use core::hash::Hash;
use num_bigint::{BigInt, BigUint, Sign, ToBigInt};
use num_traits::{CheckedAdd, CheckedSub};
use num_traits::{Signed, ToBytes, ToPrimitive, identities::Zero};

/// A dynamically sized integer type. This type is similar to [`num_bigint::BigInt`]
/// in that it allows for integers of arbitary size making it ideal for handling
/// ASN.1 `INTEGER` types, in addition this type includes small integer
/// optimisations accounting for the fact  integers decoded in ASN.1 don't
/// exceed native platform widths.
#[derive(Debug, Clone, Ord, PartialOrd)]
#[allow(missing_docs)]
pub struct Integer(IntegerKind);

macro_rules! op_or_promote {
    ($rhs:ident . $op:ident ($($args:tt)*), $promote:expr) => {
        $rhs.$op($($args)*).map(IntegerKind::Primitive).unwrap_or_else(|| IntegerKind::Variable($promote))
    }
}

/// `Integer` enum is variable-sized non-constrained integer type which uses [`isize`] for lower values to optimize performance.
#[derive(Debug, Clone, Ord, PartialOrd)]
#[allow(missing_docs)]
pub enum IntegerKind {
    Primitive(isize),
    Variable(Box<BigInt>),
}

impl Integer {
    /// Represents `0`.
    pub const ZERO: Self = Self(IntegerKind::Primitive(0));
    /// Represents `1`.
    pub const ONE: Self = Self(IntegerKind::Primitive(1));
}

impl Default for Integer {
    fn default() -> Self {
        Self::ZERO
    }
}

impl core::fmt::Display for Integer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match &self.0 {
            IntegerKind::Primitive(value) => write!(f, "{value}"),
            IntegerKind::Variable(value) => write!(f, "{value}"),
        }
    }
}

impl PartialEq for Integer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq for IntegerKind {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (IntegerKind::Primitive(lhs), IntegerKind::Primitive(rhs)) => lhs == rhs,
            (IntegerKind::Variable(lhs), IntegerKind::Variable(rhs)) => lhs == rhs,
            (IntegerKind::Primitive(lhs), IntegerKind::Variable(rhs)) => {
                lhs.to_bigint().unwrap_or_default() == **rhs
            }
            (IntegerKind::Variable(lhs), IntegerKind::Primitive(rhs)) => {
                **lhs == rhs.to_bigint().unwrap_or_default()
            }
        }
    }
}

impl Eq for Integer {}

impl Eq for IntegerKind {}

impl Hash for Integer {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl Hash for IntegerKind {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        match self {
            IntegerKind::Primitive(value) => value.hash(state),
            IntegerKind::Variable(value) => value.hash(state),
        }
    }
}

impl num_traits::CheckedAdd for Integer {
    fn checked_add(&self, other: &Self) -> Option<Self> {
        Some(Self(match (&self.0, &other.0) {
            (IntegerKind::Primitive(lhs), IntegerKind::Primitive(rhs)) => {
                op_or_promote!(lhs.checked_add(rhs), Box::new(BigInt::from(*lhs) + *rhs))
            }
            (IntegerKind::Primitive(lhs), IntegerKind::Variable(rhs)) => {
                IntegerKind::Variable(Box::new(&**rhs + lhs))
            }
            (IntegerKind::Variable(lhs), IntegerKind::Primitive(rhs)) => {
                IntegerKind::Variable(Box::new(&**lhs + *rhs))
            }
            (IntegerKind::Variable(lhs), IntegerKind::Variable(rhs)) => {
                IntegerKind::Variable(Box::new(&**lhs + &**rhs))
            }
        }))
    }
}

impl core::ops::Add for Integer {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        <Self as CheckedAdd>::checked_add(&self, &rhs).unwrap_or_default()
    }
}

macro_rules! impl_ops_integer {
    ($($t:ty),*) => {
        $(
            impl core::ops::Add<$t> for Integer {
                type Output = Self;
                fn add(self, rhs: $t) -> Self::Output {
                    Self(match self.0 {
                        IntegerKind::Primitive(lhs) => op_or_promote!(lhs.checked_add(rhs as isize), Box::new(BigInt::from(lhs) + rhs)),
                        IntegerKind::Variable(lhs) => IntegerKind::Variable(Box::new(*lhs + rhs)),
                    })
                }
            }
            impl core::ops::Sub<$t> for Integer {
                type Output = Self;
                fn sub(self, rhs: $t) -> Self::Output {
                    Self(match self.0 {
                        IntegerKind::Primitive(lhs) => op_or_promote!(lhs.checked_sub(rhs as isize), Box::new(BigInt::from(lhs) - rhs)),
                        IntegerKind::Variable(lhs) => IntegerKind::Variable(Box::new(*lhs - rhs)),
                    })
                }
            }
        )*
    };
}
macro_rules! impl_ops_integer_big {
    ($($t:ty),*) => {
        $(
            impl core::ops::Add<$t> for Integer {
                type Output = Self;
                fn add(self, rhs: $t) -> Self::Output {
                    Self(match self.0 {
                        IntegerKind::Primitive(lhs) => {
                            let value = isize::try_from(rhs).ok();
                            op_or_promote!(value.and_then(|rhs| lhs.checked_add(rhs)), Box::new(BigInt::from(lhs) + rhs))
                        }
                        IntegerKind::Variable(lhs) => {
                            IntegerKind::Variable(Box::new(*lhs + rhs))
                        }
                    })
                }
            }
            impl core::ops::Sub<$t> for Integer {
                type Output = Self;
                fn sub(self, rhs: $t) -> Self::Output {
                    Self(match self.0 {
                        IntegerKind::Primitive(lhs) => {
                            let value = isize::try_from(rhs).ok();
                            op_or_promote!(value.and_then(|rhs| lhs.checked_sub(rhs)), Box::new(BigInt::from(lhs) - rhs))
                        }
                        IntegerKind::Variable(lhs) => {
                            IntegerKind::Variable(Box::new(*lhs - rhs))
                        }
                    })
                }
            }
        )*
    };
}

// Safely casted to isize without truncation, used on 32-bit targets
#[cfg(target_pointer_width = "32")]
impl_ops_integer!(u8, u16, i8, i16, i32, isize);
#[cfg(target_pointer_width = "32")]
impl_ops_integer_big!(u32, i64);
// Safely casted to isize without truncation, used on 64-bit targets
#[cfg(target_pointer_width = "64")]
impl_ops_integer!(u8, u16, u32, i8, i16, i32, i64, isize);

// Never safely casted for isize variant, used on all targets
impl_ops_integer_big!(u64, u128, usize, i128);

impl num_traits::CheckedSub for Integer {
    fn checked_sub(&self, other: &Self) -> Option<Self> {
        Some(Self(match (&self.0, &other.0) {
            (IntegerKind::Primitive(lhs), IntegerKind::Primitive(rhs)) => {
                op_or_promote!(lhs.checked_sub(rhs), Box::new(BigInt::from(*lhs) - *rhs))
            }
            (IntegerKind::Primitive(lhs), IntegerKind::Variable(rhs)) => {
                IntegerKind::Variable(Box::new(BigInt::from(*lhs) - &**rhs))
            }
            (IntegerKind::Variable(lhs), IntegerKind::Primitive(rhs)) => {
                IntegerKind::Variable(Box::new(&**lhs - *rhs))
            }
            (IntegerKind::Variable(lhs), IntegerKind::Variable(rhs)) => {
                IntegerKind::Variable(Box::new(&**lhs - &**rhs))
            }
        }))
    }
}

impl core::ops::Sub for Integer {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        <Self as CheckedSub>::checked_sub(&self, &rhs).unwrap_or_default()
    }
}

impl ToPrimitive for Integer {
    fn to_i64(&self) -> Option<i64> {
        match &self.0 {
            IntegerKind::Primitive(value) => Some(*value as i64),
            IntegerKind::Variable(value) => value.to_i64(),
        }
    }
    fn to_u64(&self) -> Option<u64> {
        match &self.0 {
            #[allow(clippy::cast_sign_loss)] // Primitive is not larger than `isize`
            IntegerKind::Primitive(value) => (*value >= 0).then_some(*value as u64),
            IntegerKind::Variable(value) => value.to_u64(),
        }
    }
    fn to_i128(&self) -> Option<i128> {
        match &self.0 {
            IntegerKind::Primitive(value) => Some(*value as i128),
            IntegerKind::Variable(value) => value.to_i128(),
        }
    }
}

macro_rules! impl_from_integer_as_prim {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Integer {
                fn from(value: $t) -> Self {
                    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)] // We don't support 16-bit arch
                    Self(IntegerKind::Primitive(value as isize))
                }
            }
        )*
    };
}

macro_rules! impl_from_integer_as_big {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Integer {
                fn from(value: $t) -> Self {
                    Self(op_or_promote!(value.to_isize(), Box::new(BigInt::from(value))))
                }
            }
        )*
    };
}
#[cfg(target_pointer_width = "32")]
impl_from_integer_as_prim!(u8, u16, i8, i16, i32, isize);
#[cfg(target_pointer_width = "32")]
impl_from_integer_as_big!(u32, i64);
#[cfg(target_pointer_width = "64")]
impl_from_integer_as_prim!(u8, u16, u32, i8, i16, i32, i64, isize);
// Never fit for isize variant, used on all targets
impl_from_integer_as_big!(u64, u128, i128, usize, BigInt);

impl From<Integer> for BigInt {
    fn from(value: Integer) -> Self {
        match value.0 {
            IntegerKind::Primitive(value) => value.to_bigint().unwrap_or_default(),
            IntegerKind::Variable(value) => *value,
        }
    }
}

impl ToBigInt for Integer {
    fn to_bigint(&self) -> Option<BigInt> {
        match &self.0 {
            IntegerKind::Primitive(value) => Some(BigInt::from(*value)),
            IntegerKind::Variable(value) => Some(*value.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TryFromIntegerError {
    original: BigInt,
}

impl TryFromIntegerError {
    fn new(original: BigInt) -> Self {
        TryFromIntegerError { original }
    }
    fn __description(&self) -> &str {
        "out of range conversion regarding integer conversion attempted"
    }
    pub fn into_original(self) -> BigInt {
        self.original
    }
}

impl alloc::fmt::Display for TryFromIntegerError {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        self.__description().fmt(f)
    }
}
macro_rules! impl_try_from_integer {
    ($($t:ty),*) => {
        $(
            impl core::convert::TryFrom<Integer> for $t {
                type Error = TryFromIntegerError;
                fn try_from(value: Integer) -> Result<Self, Self::Error> {
                    Self::try_from(&value)
                }
            }
            impl core::convert::TryFrom<&Integer> for $t {
                type Error = TryFromIntegerError;
                fn try_from(value: &Integer) -> Result<Self, Self::Error> {
                    match &value.0 {
                        IntegerKind::Primitive(value) => (*value).try_into().map_err(|_| TryFromIntegerError::new(value.to_bigint().unwrap_or_default())),
                        IntegerKind::Variable(value) => (**value).clone().try_into().map_err(|_| TryFromIntegerError::new(*value.clone())),
                    }
                }
            }
        )*
    };
}
impl_try_from_integer!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize
);

/// An integer which has encoded constraint range between `START` and `END`.
#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ConstrainedInteger<const START: i128, const END: i128>(pub(crate) Integer);

impl<const START: i128, const END: i128> ConstrainedInteger<START, END> {
    /// Unchecked `new` where constraint boundaries are not checked - used only in tests.
    #[cfg(test)]
    pub(crate) fn new<T: Into<Integer>>(value: T) -> Self {
        Self(value.into())
    }
}

impl<const START: i128, const END: i128> AsnType for ConstrainedInteger<START, END> {
    const TAG: Tag = Tag::INTEGER;
    const CONSTRAINTS: Constraints =
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

macro_rules! impl_try_from_integer_constrained {
    ($($t:ty),*) => {
        $(
            impl<const START: i128, const END: i128> TryFrom<$t> for ConstrainedInteger<START, END> {
                type Error = TryFromIntegerError;
                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    if value as i128 >= START && value as i128 <= END {
                        Ok(Self(value.into()))
                    } else {
                        Err(TryFromIntegerError::new(value.into()))
                    }
                }
            }
        )*
    }
}
impl_try_from_integer_constrained!(isize, i32, i64, i128);

/// Represents a integer type in Rust that can be decoded or encoded into any
/// ASN.1 codec.
pub trait IntegerType:
    Sized
    + Clone
    + core::fmt::Debug
    + core::fmt::Display
    + Default
    + TryInto<i64>
    + TryInto<i128>
    + TryInto<isize>
    + TryFrom<i64>
    + TryFrom<i128>
    + TryFrom<isize>
    + TryFrom<BigInt>
    + Into<BigInt>
    + ToBigInt
    + num_traits::CheckedAdd
    + num_traits::CheckedSub
    + core::cmp::PartialOrd
    + core::cmp::PartialEq
    + num_traits::ToPrimitive
{
    /// The bit level width of the integer (e.g. u32 is 32 bits).
    const WIDTH: u32;
    /// The byte level width of the integer(e.g. u32 is 4 bytes).
    const BYTE_WIDTH: usize = Self::WIDTH as usize / 8;
    /// Represents `0` in a given integer.
    const ZERO: Self;
    /// `Self` as an unsigned type with the same width.
    type UnsignedPair: IntegerType;
    /// `Self` as a signed type with one type size larger to prevent truncation, in case `Self` is unsigned. (e.g. u8 -> i16)
    /// Truncation would happen if unsigned type is converted to signed bytes with the same size.
    type SignedPair: IntegerType;

    /// Attempts to convert the input data matching the given codec into [Self].
    ///
    /// # Errors
    /// If the data doesn't represent a valid integer in the given codec.
    fn try_from_bytes(input: &[u8], codec: crate::Codec)
    -> Result<Self, crate::error::DecodeError>;

    /// Attempts to convert the input data (assuming signed bytes) matching the given codec into [Self].
    ///
    /// # Errors
    /// If the data doesn't represent a valid integer in the given codec.
    fn try_from_signed_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError>;

    /// Attempts to convert the input data (assuming unsigned bytes) matching the given codec into [Self].
    ///
    /// # Errors
    /// If the data doesn't represent a valid integer in the given codec.
    fn try_from_unsigned_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError>;

    /// Returns a minimum number of signed Big Endian bytes needed to present the integer, byte count defined by `usize`.
    fn to_signed_bytes_be(&self) -> (impl AsRef<[u8]>, usize);

    /// Returns minimum number defined by `usize` of unsigned Big-endian bytes needed to present the the integer.
    fn to_unsigned_bytes_be(&self) -> (impl AsRef<[u8]>, usize);

    /// Makes it possible to add unsigned integer to signed integer
    /// This is mainly used on UPER, in order to add unsigned offset into signed constrained minimum to get the resulting value as correct type.
    /// Casting will change the "value", but same width makes the result ending to be correct.
    fn wrapping_unsigned_add(self, other: Self::UnsignedPair) -> Self;
    /// Whether the integer value is negative or not.
    fn is_negative(&self) -> bool;
    /// Whether the integer type is signed or not.
    fn is_signed(&self) -> bool;
    /// Convert the underlying integer type into rasn ASN.1 `Integer` type.
    fn to_integer(self) -> Integer;
}

trait MinFixedSizeIntegerBytes:
    IntegerType + ToBytes + Copy + core::ops::Shl<u32, Output = Self>
{
    /// Encodes the `N` byte integer as big-endian bytes without the leading
    /// bytes that only repeat its sign (zeros for an unsigned value), and
    /// returns them at the front of the array with their number.
    #[inline(always)]
    fn needed_as_be_bytes<const N: usize>(&self, signed: bool) -> ([u8; N], usize) {
        let needed = if signed {
            self.signed_bytes_needed()
        } else {
            self.unsigned_bytes_needed()
        };
        // Shifting the value up moves its needed bytes to the front of the
        // big-endian representation.
        let shifted = *self << ((N - needed) * 8) as u32;
        let bytes: [u8; N] = shifted.to_be_bytes().as_ref().try_into().unwrap_or([0; N]);
        (bytes, needed)
    }
    /// Finds the minimum number of bytes needed to present the unsigned integer. (in order to drop unecessary leading zeros or ones)
    fn unsigned_bytes_needed(&self) -> usize;

    /// Finds the minimum number of bytes needed to present the signed integer. (in order to drop unecessary leading zeros or ones)
    fn signed_bytes_needed(&self) -> usize;
}

macro_rules! integer_type_impl {
    ((signed $t1:ty, $t2:ty), $($ts:tt)*) => {
        impl IntegerType for $t1 {
            const WIDTH: u32 = <$t1>::BITS;
            const ZERO: $t1 = 0 as $t1;
            type UnsignedPair = $t2;
            type SignedPair = $t1;

            #[inline(always)]
            fn try_from_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                Self::try_from_signed_bytes(input, codec)
            }

            #[inline(always)]
            fn try_from_signed_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                const BYTE_SIZE: usize = core::mem::size_of::<$t1>();
                if input.is_empty() {
                    return Err(crate::error::DecodeError::unexpected_empty_input(codec));
                }
                if input.len() > BYTE_SIZE {
                    return Err(crate::error::DecodeError::integer_overflow(<$t1>::BITS, codec));
                }
                // The first octet carries the sign: place it at the top and
                // extend it with an arithmetic shift, then add the rest.
                let mut result = (input[0] as $t1) << ((BYTE_SIZE - 1) * 8);
                result >>= (BYTE_SIZE - input.len()) * 8;
                for (i, &byte) in input.iter().skip(1).enumerate() {
                    result |= (byte as $t1) << (8 * (input.len() - 2 - i));
                }
                Ok(result)
            }

            #[inline(always)]
            fn try_from_unsigned_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                <$t1>::try_from(<$t2>::try_from_bytes(input, codec)?)
                    .map_err(|_| {
                    crate::error::DecodeError::integer_type_conversion_failed(alloc::format!("Failed to create signed integer from unsigned bytes, target bit-size {}, with bytes {:?}", <$t1>::BITS, input).into(), codec)
                })

            }
            #[inline(always)]
            fn to_signed_bytes_be(&self) -> (impl AsRef<[u8]>, usize) {
                const N: usize = core::mem::size_of::<$t1>();
                self.needed_as_be_bytes::<N>( true)
            }
            #[inline(always)]
            fn to_unsigned_bytes_be(&self) -> (impl AsRef<[u8]>, usize) {
                const N: usize = core::mem::size_of::<$t2>();
                (*self as $t2).needed_as_be_bytes::<N>( false)
            }

            fn wrapping_unsigned_add(self, other: $t2) -> Self {
                self.wrapping_add(other as $t1)
            }
            fn is_negative(&self) -> bool {
                <Self as Signed>::is_negative(self)
            }
            fn is_signed(&self) -> bool {
                true
            }

            fn to_integer(self) -> Integer {
                Integer(op_or_promote!(self.to_isize(), Box::new(self.to_bigint().unwrap_or_default())))
            }
        }
        impl MinFixedSizeIntegerBytes for $t1 {

            #[inline(always)]
            fn unsigned_bytes_needed(&self) -> usize {
                (*self as $t2).unsigned_bytes_needed()
            }
            #[inline(always)]
            fn signed_bytes_needed(&self) -> usize {
                let leading_bits = if Signed::is_negative(self) {
                    self.leading_ones() as usize
                } else {
                    self.leading_zeros() as usize
                };
                let full_bytes = Self::BYTE_WIDTH - leading_bits / 8;
                let extra_byte = (leading_bits % 8 == 0) as usize;
                full_bytes + extra_byte

            }
        }

        integer_type_impl!($($ts)*);
    };
    ((unsigned $t1:ty, $t2:ty), $($ts:tt)*) => {
        impl IntegerType for $t1 {
            const WIDTH: u32 = <$t1>::BITS;
            const ZERO: $t1 = 0 as $t1;
            type UnsignedPair = $t1;
            type SignedPair = $t2;

            #[inline(always)]
            fn try_from_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                Self::try_from_unsigned_bytes(input, codec)
            }

            #[inline(always)]
            fn try_from_signed_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                // A two's complement value fits once its sign is known to be
                // positive: the octet that only keeps the sign bit clear is
                // then not part of the magnitude.
                match input {
                    [] => Err(crate::error::DecodeError::unexpected_empty_input(codec)),
                    [first, ..] if first & 0x80 != 0 => Err(
                        crate::error::DecodeError::integer_type_conversion_failed(alloc::format!("Failed to create unsigned integer from signed bytes, target bit-size {}, with bytes: {:?}", <$t1>::BITS, input).into(), codec)
                    ),
                    [0] => Ok(0),
                    [0, magnitude @ ..] => Self::try_from_unsigned_bytes(magnitude, codec),
                    magnitude => Self::try_from_unsigned_bytes(magnitude, codec),
                }
            }

            #[inline(always)]
            fn try_from_unsigned_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                const BYTE_SIZE: usize = core::mem::size_of::<$t1>();
                if input.is_empty() {
                    return Err(crate::error::DecodeError::unexpected_empty_input(codec));
                }
                if input.len() > BYTE_SIZE {
                    return Err(crate::error::DecodeError::integer_overflow(<$t1>::BITS, codec));
                }
                let start_shift = (input.len() - 1) * 8;
                let mut result: $t1 = 0;
                for (i, &byte) in input.iter().enumerate() {
                    result |= (byte as $t1) << (start_shift - i * 8);
                }
                Ok(result)
            }

            // Getting signed bytes of an unsigned integer is challenging, because we don't want to truncate the value
            // Signed bytes of u8::MAX for example takes more bytes than unsigned bytes.
            // As a result, we cast the type to next fitting signed type if possible and use the size of its to define the sign bytes.
            #[inline(always)]
            fn to_signed_bytes_be(&self) -> (impl AsRef<[u8]>, usize) {
                const N: usize = core::mem::size_of::<$t2>();
                (*self as $t2).needed_as_be_bytes::<N>(true)
            }
            #[inline(always)]
            fn to_unsigned_bytes_be(&self) -> (impl AsRef<[u8]>, usize) {
                const N: usize = core::mem::size_of::<$t1>();
                self.needed_as_be_bytes::<N>(false)
            }

            fn wrapping_unsigned_add(self, other: $t1) -> Self {
                self.wrapping_add(other)
            }
            fn is_negative(&self) -> bool {
                false
            }
            fn is_signed(&self) -> bool {
                false
            }
            fn to_integer(self) -> Integer {
                Integer(op_or_promote!(self.to_isize(), Box::new(self.to_bigint().unwrap_or_default())))
            }
        }
        impl MinFixedSizeIntegerBytes for $t1 {
            #[inline(always)]
            fn unsigned_bytes_needed(&self) -> usize {
                if self.is_zero() {
                    1
                } else {
                    let significant_bits = Self::WIDTH as usize - self.leading_zeros() as usize;
                    significant_bits.div_ceil(8)
                }
            }
            #[inline(always)]
            fn signed_bytes_needed(&self) -> usize {
                (*self as $t2).signed_bytes_needed()
            }
        }

        integer_type_impl!($($ts)*);
    };
    (,) => {};
    () => {};
}

integer_type_impl!(
    (unsigned u8, i16),
    (signed i8, u8),
    (unsigned u16, i32),
    (signed i16, u16),
    (unsigned u32, i64),
    (signed i32, u32),
    (unsigned u64, i128),
    (signed i64, u64),
    // Will truncate on i128 on large numbers
    (unsigned u128, i128),
    (signed i128, u128),
    (unsigned usize, i128),
    (signed isize, usize),
);

/// Drops the leading octets of a two's complement integer that only repeat
/// its sign: an octet of all ones or all zeros whose successor starts with
/// the same bit (X.690 §8.3.2).
pub(crate) fn without_redundant_sign_octets(contents: &[u8]) -> &[u8] {
    let sign = contents.first().map_or(0, |first| first & 0x80);
    let redundant = contents
        .windows(2)
        .take_while(|pair| pair[0] == if sign == 0 { 0x00 } else { 0xFF } && pair[1] & 0x80 == sign)
        .count();
    &contents[redundant..]
}

/// The widest integer converted through a stack buffer, in octets; a wider
/// one goes through the heap.
const STACK_INTEGER_OCTETS: usize = 64;

/// Reads a big-endian integer, in two's complement when `signed`. One that
/// fits the stack buffer is reversed there for `BigUint::from_bytes_le`,
/// which saves the copies `from_bytes_be` and `from_signed_bytes_be` make.
fn big_int_from_bytes_be(bytes: &[u8], signed: bool) -> BigInt {
    let negative = signed && bytes.first().is_some_and(|first| first & 0x80 != 0);
    let mut buffer = [0; STACK_INTEGER_OCTETS];
    let Some(reversed) = buffer.get_mut(..bytes.len()) else {
        return if signed {
            BigInt::from_signed_bytes_be(bytes)
        } else {
            BigUint::from_bytes_be(bytes).into()
        };
    };
    let octets = reversed.iter_mut().zip(bytes.iter().rev());
    if negative {
        // The magnitude of a negative value is its two's complement: every
        // octet inverted, and one added from the least significant end.
        let mut carry = true;
        for (octet, &source) in octets {
            (*octet, carry) = (!source).overflowing_add(u8::from(carry));
        }
    } else {
        for (octet, &source) in octets {
            *octet = source;
        }
    }
    let sign = if negative { Sign::Minus } else { Sign::Plus };
    BigInt::from_biguint(sign, BigUint::from_bytes_le(reversed))
}

/// The octets of a `BigInt`: on the stack when the value fits the buffer.
enum BigIntegerBytes {
    Stack {
        octets: [u8; STACK_INTEGER_OCTETS + 1],
        start: usize,
    },
    Heap(alloc::vec::Vec<u8>),
}

impl AsRef<[u8]> for BigIntegerBytes {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Stack { octets, start } => &octets[*start..],
            Self::Heap(octets) => octets,
        }
    }
}

/// The magnitude, most significant octet first, at the end of a buffer, and
/// the index of the clear octet before its digits, where a sign would go;
/// `None` when it does not fit.
fn magnitude_octets(magnitude: &BigUint) -> Option<([u8; STACK_INTEGER_OCTETS + 1], usize)> {
    let digits = magnitude.iter_u64_digits();
    if digits.len() > STACK_INTEGER_OCTETS / size_of::<u64>() {
        return None;
    }
    let mut octets = [0; STACK_INTEGER_OCTETS + 1];
    let sign = octets.len() - 1 - digits.len() * size_of::<u64>();
    // The digits come least significant first and fill the buffer from its end.
    let (chunks, _) = octets[sign + 1..].as_chunks_mut::<{ size_of::<u64>() }>();
    for (chunk, digit) in chunks.iter_mut().rev().zip(digits) {
        *chunk = digit.to_be_bytes();
    }
    Some((octets, sign))
}

/// The two's complement of `value` in the fewest octets (X.690 §8.3.2).
fn big_int_to_signed_bytes_be(value: &BigInt) -> BigIntegerBytes {
    let Some((mut octets, sign)) = magnitude_octets(value.magnitude()) else {
        return BigIntegerBytes::Heap(value.to_signed_bytes_be());
    };
    if value.sign() == Sign::Minus {
        let mut carry = true;
        for octet in octets[sign..].iter_mut().rev() {
            (*octet, carry) = (!*octet).overflowing_add(u8::from(carry));
        }
    }
    let start = octets.len() - without_redundant_sign_octets(&octets[sign..]).len();
    BigIntegerBytes::Stack { octets, start }
}

/// The magnitude of `value` in the fewest octets, and zero for a negative
/// value.
fn big_int_to_unsigned_bytes_be(value: &BigInt) -> BigIntegerBytes {
    let octets = match value.sign() {
        Sign::Minus => Some(([0; STACK_INTEGER_OCTETS + 1], STACK_INTEGER_OCTETS)),
        Sign::NoSign | Sign::Plus => magnitude_octets(value.magnitude()),
    };
    let Some((octets, sign)) = octets else {
        return BigIntegerBytes::Heap(value.magnitude().to_bytes_be());
    };
    let start = octets[sign..]
        .iter()
        .position(|&octet| octet != 0)
        .map_or(octets.len() - 1, |leading| sign + leading);
    BigIntegerBytes::Stack { octets, start }
}

impl IntegerType for BigInt {
    const WIDTH: u32 = u32::MAX;
    const ZERO: BigInt = BigInt::ZERO;
    type UnsignedPair = Self;
    type SignedPair = Self;

    #[inline(always)]
    fn try_from_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError> {
        if input.is_empty() {
            return Err(crate::error::DecodeError::unexpected_empty_input(codec));
        }
        Ok(big_int_from_bytes_be(input, true))
    }

    #[inline(always)]
    fn try_from_signed_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError> {
        Self::try_from_bytes(input, codec)
    }

    #[inline(always)]
    fn try_from_unsigned_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError> {
        if input.is_empty() {
            return Err(crate::error::DecodeError::unexpected_empty_input(codec));
        }
        Ok(big_int_from_bytes_be(input, false))
    }
    #[inline(always)]
    fn to_signed_bytes_be(&self) -> (impl AsRef<[u8]>, usize) {
        let bytes = big_int_to_signed_bytes_be(self);
        let len = bytes.as_ref().len();
        (bytes, len)
    }
    #[inline(always)]
    fn to_unsigned_bytes_be(&self) -> (impl AsRef<[u8]>, usize) {
        let bytes = big_int_to_unsigned_bytes_be(self);
        let len = bytes.as_ref().len();
        (bytes, len)
    }

    fn wrapping_unsigned_add(self, other: Self) -> Self {
        self + other
    }
    fn is_negative(&self) -> bool {
        <Self as Signed>::is_negative(self)
    }
    fn is_signed(&self) -> bool {
        true
    }
    fn to_integer(self) -> Integer {
        Integer(IntegerKind::Variable(Box::new(self)))
    }
}
/// The octets of an [`Integer`], from whichever kind it holds.
enum IntegerBytes<T: AsRef<[u8]>> {
    Primitive([u8; core::mem::size_of::<isize>()]),
    Variable(T),
}

impl<T: AsRef<[u8]>> AsRef<[u8]> for IntegerBytes<T> {
    fn as_ref(&self) -> &[u8] {
        match self {
            IntegerBytes::Primitive(bytes) => bytes,
            IntegerBytes::Variable(bytes) => bytes.as_ref(),
        }
    }
}

impl IntegerType for Integer {
    const WIDTH: u32 = u32::MAX;
    const ZERO: Integer = Integer::ZERO;
    type UnsignedPair = Self;
    type SignedPair = Self;

    #[inline(always)]
    fn try_from_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError> {
        if input.is_empty() {
            return Err(crate::error::DecodeError::unexpected_empty_input(codec));
        }
        // Two's complement fits `isize` in as many octets as `isize` has.
        if input.len() <= size_of::<isize>() {
            isize::try_from_bytes(input, codec).map(|value| Self(IntegerKind::Primitive(value)))
        } else {
            BigInt::try_from_bytes(input, codec)
                .map(|value| Self(IntegerKind::Variable(Box::new(value))))
        }
    }

    #[inline(always)]
    fn try_from_signed_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError> {
        Self::try_from_bytes(input, codec)
    }

    #[inline(always)]
    fn try_from_unsigned_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError> {
        let Some(&first) = input.first() else {
            return Err(crate::error::DecodeError::unexpected_empty_input(codec));
        };
        // A magnitude fits `isize` in as many octets only with the top bit clear.
        if input.len() < size_of::<isize>() || input.len() == size_of::<isize>() && first < 0x80 {
            isize::try_from_unsigned_bytes(input, codec)
                .map(|value| Self(IntegerKind::Primitive(value)))
        } else {
            BigInt::try_from_unsigned_bytes(input, codec)
                .map(|value| Self(IntegerKind::Variable(Box::new(value))))
        }
    }
    #[inline(always)]
    fn to_signed_bytes_be(&self) -> (impl AsRef<[u8]>, usize) {
        match &self.0 {
            IntegerKind::Primitive(value) => {
                let (bytes, len) = value.needed_as_be_bytes(true);
                (IntegerBytes::Primitive(bytes), len)
            }
            IntegerKind::Variable(value) => {
                let (bytes, len) = <BigInt as IntegerType>::to_signed_bytes_be(value);
                (IntegerBytes::Variable(bytes), len)
            }
        }
    }
    #[inline(always)]
    fn to_unsigned_bytes_be(&self) -> (impl AsRef<[u8]>, usize) {
        match &self.0 {
            IntegerKind::Primitive(value) => {
                let (bytes, len) = (*value as usize).needed_as_be_bytes(false);
                (IntegerBytes::Primitive(bytes), len)
            }
            IntegerKind::Variable(value) => {
                let (bytes, len) = <BigInt as IntegerType>::to_unsigned_bytes_be(value);
                (IntegerBytes::Variable(bytes), len)
            }
        }
    }

    fn wrapping_unsigned_add(self, other: Self) -> Self {
        self + other
    }
    fn is_negative(&self) -> bool {
        match &self.0 {
            IntegerKind::Primitive(value) => <isize as IntegerType>::is_negative(value),
            IntegerKind::Variable(value) => <BigInt as IntegerType>::is_negative(value),
        }
    }
    fn is_signed(&self) -> bool {
        true
    }
    fn to_integer(self) -> Integer {
        self
    }
}

#[cfg(test)]
macro_rules! assert_integer_round_trip {
    ($t:ty, $value:expr, $expected_unsigned:expr, $expected_signed:expr) => {{
        let value = <$t>::try_from($value).unwrap();

        // Test unsigned bytes
        let (unsigned_bytes, unsigned_needed) = value.to_unsigned_bytes_be();
        assert_eq!(
            &unsigned_bytes.as_ref()[..unsigned_needed],
            $expected_unsigned,
            "Unsigned bytes mismatch for {}",
            stringify!($value)
        );

        // Only test unsigned round-trip for non-negative values or unsigned types
        #[allow(unused_comparisons)]
        if $value >= 0 || stringify!($t).starts_with('u') {
            assert_eq!(
                <$t>::try_from_unsigned_bytes(
                    &unsigned_bytes.as_ref()[..unsigned_needed],
                    crate::Codec::Oer
                )
                .ok(),
                Some(value),
                "Round-trip failed for unsigned bytes of {}",
                stringify!($value)
            );
        }

        // Test signed bytes
        let (signed_bytes, signed_needed) = value.to_signed_bytes_be();
        assert_eq!(
            &signed_bytes.as_ref()[..signed_needed],
            $expected_signed,
            "Signed bytes mismatch for {}",
            stringify!($value)
        );

        // Always test signed round-trip
        assert_eq!(
            <$t>::try_from_signed_bytes(&signed_bytes.as_ref()[..signed_needed], crate::Codec::Oer)
                .ok(),
            Some(value),
            "Round-trip failed for signed bytes of {}",
            stringify!($value)
        );
        // Round trip with Integer type should work for any type with all values
        let integer = Integer::from($value);
        let (bytes, needed) = integer.to_signed_bytes_be();
        assert_eq!(
            Integer::try_from_signed_bytes(&bytes.as_ref()[..needed], crate::Codec::Oer).ok(),
            Some($value.into()),
            "Round-trip failed for Integer({})",
            stringify!($value)
        );
        // Check that encoding matches the signed expected bytes for Integer type
        assert_eq!(
            &bytes.as_ref()[..needed],
            $expected_signed,
            "Signed bytes mismatch for Integer({})",
            stringify!($value)
        );
    }};
}

macro_rules! test_integer_conversions_and_operations {
    ($($t:ident),*) => {
        #[cfg(test)]
        mod tests {
            use crate::prelude::*;
            use super::*;

            $(
                #[test]
                fn $t() {
                    let min = <$t>::MIN as i128;
                    let max = <$t>::MAX as u128;

                    if min >= isize::MIN as i128 {
                        assert!(matches!(min.into(), Integer(IntegerKind::Primitive(_))));
                    } else {
                        assert!(matches!(min.into(), Integer(IntegerKind::Variable(_))));
                    }
                    if max <= isize::MAX as u128 {
                        assert!(matches!(max.into(), Integer(IntegerKind::Primitive(_))));
                    } else {
                        assert!(matches!(max.into(), Integer(IntegerKind::Variable(_))));
                    }

                    // Test positive values
                    assert_integer_round_trip!($t, 1, &[1], &[1]);

                    // Test some signed maximum values, should be identical to unsigned
                    if <$t>::MAX as u128 >= i8::MAX as u128 {
                        assert_integer_round_trip!($t, i8::MAX as $t, &[127], &[127]);
                    }
                    // Even if the type is wider than 2 bytes, the value remains the same
                    if <$t>::MAX as u128 >= i16::MAX as u128 {
                        assert_integer_round_trip!($t, i16::MAX as $t, &[127, 255], &[127, 255]);
                        // 127 + 1 results to leading zero for signed types
                        assert_integer_round_trip!($t, 128, &[128], &[0, 128]);
                    }
                    if <$t>::MAX as u128 >= i32::MAX as u128 {
                        assert_integer_round_trip!($t, i32::MAX as $t, &[127, 255, 255, 255], &[127, 255, 255, 255]);
                        // 32_767 + 1 results to leading zero for signed types
                        assert_integer_round_trip!($t, (i16::MAX as $t + 1), &[128, 0], &[0, 128, 0]);
                    }
                    if <$t>::MAX as u128 >= i64::MAX as u128 {
                        assert_integer_round_trip!($t, i64::MAX as $t, &[127, 255, 255, 255, 255, 255, 255, 255], &[127, 255, 255, 255, 255, 255, 255, 255]);
                    }


                    // Test negative values for signed types
                    match stringify!($t) {
                        "i8" => {
                            assert_integer_round_trip!($t, -1, &[255], &[255]);
                        },
                        "i16" => {
                            assert_integer_round_trip!($t, -1, &[255, 255], &[255]);
                        },
                        "i32" => {
                            assert_integer_round_trip!($t, -1, &[255, 255, 255, 255], &[255]);
                        },
                        "i64" => {
                            assert_integer_round_trip!($t, -1, &[255, 255, 255, 255, 255, 255, 255, 255], &[255]);
                        },
                        _ => {},
                    }
                }
            )*

            #[test]
            fn test_overflow_addition() {
                let max = Integer(IntegerKind::Primitive(isize::MAX));
                let one = Integer(IntegerKind::Primitive(1));
                let result = max + one;
                assert!(matches!(result, Integer(IntegerKind::Variable(_))));
            }

            #[test]
            fn test_overflow_subtraction() {
                let min = Integer(IntegerKind::Primitive(isize::MIN));
                let one = Integer(IntegerKind::Primitive(1));
                let result = min - one;
                assert!(matches!(result, Integer(IntegerKind::Variable(_))));
            }

            #[test]
            fn test_primitive_to_i64() {
                let zero = Integer(IntegerKind::Primitive(0));
                let positive = Integer(IntegerKind::Primitive(42));
                let negative = Integer(IntegerKind::Primitive(-42));
                let max = Integer(IntegerKind::Primitive(isize::MAX));
                let min = Integer(IntegerKind::Primitive(isize::MIN));

                assert_eq!(zero.to_i64(), Some(0i64));
                assert_eq!(positive.to_i64(), Some(42i64));
                assert_eq!(negative.to_i64(), Some(-42i64));
                assert_eq!(max.to_i64(), Some(isize::MAX as i64));
                assert_eq!(min.to_i64(), Some(isize::MIN as i64));
            }
            #[test]
            fn test_primitive_to_u64() {
                let zero = Integer(IntegerKind::Primitive(0));
                let positive = Integer(IntegerKind::Primitive(42));
                let negative = Integer(IntegerKind::Primitive(-42));
                let max = Integer(IntegerKind::Primitive(isize::MAX));

                assert_eq!(zero.to_u64(), Some(0u64));
                assert_eq!(positive.to_u64(), Some(42u64));
                assert_eq!(negative.to_u64(), None); // Negative
                assert_eq!(max.to_u64(), Some(isize::MAX as u64));
            }
            #[test]
            fn test_primitive_to_i128() {
                let zero = Integer(IntegerKind::Primitive(0));
                let positive = Integer(IntegerKind::Primitive(42));
                let negative = Integer(IntegerKind::Primitive(-42));
                let max = Integer(IntegerKind::Primitive(isize::MAX));
                let min = Integer(IntegerKind::Primitive(isize::MIN));

                assert_eq!(zero.to_i128(), Some(0i128));
                assert_eq!(positive.to_i128(), Some(42i128));
                assert_eq!(negative.to_i128(), Some(-42i128));
                assert_eq!(max.to_i128(), Some(isize::MAX as i128));
                assert_eq!(min.to_i128(), Some(isize::MIN as i128));
            }
            #[test]
            fn test_variable_to_i64() {
                let zero = Integer(IntegerKind::Variable(Box::new(BigInt::from(0))));
                let positive = Integer(IntegerKind::Variable(Box::new(BigInt::from(100))));
                let negative = Integer(IntegerKind::Variable(Box::new(BigInt::from(-100))));
                let large_positive = Integer(IntegerKind::Variable(Box::new(BigInt::from(i64::MAX) + 1)));
                let large_negative = Integer(IntegerKind::Variable(Box::new(BigInt::from(i64::MIN) - 1)));

                assert_eq!(zero.to_i64(), Some(0i64));
                assert_eq!(positive.to_i64(), Some(100i64));
                assert_eq!(negative.to_i64(), Some(-100i64));
                assert_eq!(large_positive.to_i64(), None); // Too large for i64
                assert_eq!(large_negative.to_i64(), None); // Too small for i64
            }

            #[test]
            fn test_variable_to_u64() {
                let zero = Integer(IntegerKind::Variable(Box::new(BigInt::from(0))));
                let positive = Integer(IntegerKind::Variable(Box::new(BigInt::from(100))));
                let negative = Integer(IntegerKind::Variable(Box::new(BigInt::from(-100))));
                let large_positive = Integer(IntegerKind::Variable(Box::new(BigInt::from(u64::MAX) + 1)));

                assert_eq!(zero.to_u64(), Some(0u64));
                assert_eq!(positive.to_u64(), Some(100u64));
                assert_eq!(negative.to_u64(), None); // Negative BigInt to u64
                assert_eq!(large_positive.to_u64(), None); // Too large for u64
            }

            #[test]
            fn test_variable_to_i128() {
                let zero = Integer(IntegerKind::Variable(Box::new(BigInt::from(0))));
                let positive = Integer(IntegerKind::Variable(Box::new(BigInt::from(1000))));
                let negative = Integer(IntegerKind::Variable(Box::new(BigInt::from(-1000))));
                let very_large_positive = Integer(IntegerKind::Variable(Box::new(BigInt::from(i128::MAX) + BigInt::from(1))));
                let very_large_negative = Integer(IntegerKind::Variable(Box::new(BigInt::from(i128::MIN) - BigInt::from(1))));


                assert_eq!(zero.to_i128(), Some(0i128));
                assert_eq!(positive.to_i128(), Some(1000i128));
                assert_eq!(negative.to_i128(), Some(-1000i128));
                assert_eq!(very_large_positive.to_i128(), None);
                assert_eq!(very_large_negative.to_i128(), None);
            }

            #[test]
            fn unsigned_octets_of_a_variable_integer_are_its_magnitude() {
                let value = Integer::from(u128::MAX);
                let (bytes, len) = value.to_unsigned_bytes_be();
                assert_eq!(&bytes.as_ref()[..len], [0xFF; 16]);
            }

            #[test]
            fn representation_follows_the_octet_count() {
                let width = size_of::<isize>();
                let codec = crate::Codec::Ber;

                // Two's complement: `isize` octets fit, one more does not.
                let minimum = [&[0x80][..], &alloc::vec![0x00; width - 1]].concat();
                assert!(matches!(
                    Integer::try_from_bytes(&minimum, codec).unwrap(),
                    Integer(IntegerKind::Primitive(isize::MIN))
                ));
                let beyond = [&[0x00, 0x80][..], &alloc::vec![0x00; width - 1]].concat();
                let beyond = Integer::try_from_bytes(&beyond, codec).unwrap();
                assert!(matches!(beyond, Integer(IntegerKind::Variable(_))));
                assert_eq!(beyond.to_i128(), Some(isize::MAX as i128 + 1));

                // A magnitude: `isize` octets fit only with the top bit clear.
                let maximum = [&[0x7F][..], &alloc::vec![0xFF; width - 1]].concat();
                assert!(matches!(
                    Integer::try_from_unsigned_bytes(&maximum, codec).unwrap(),
                    Integer(IntegerKind::Primitive(isize::MAX))
                ));
                let top_bit = [&[0x80][..], &alloc::vec![0x00; width - 1]].concat();
                let top_bit = Integer::try_from_unsigned_bytes(&top_bit, codec).unwrap();
                assert!(matches!(top_bit, Integer(IntegerKind::Variable(_))));
                assert_eq!(top_bit.to_i128(), Some(isize::MAX as i128 + 1));
            }
        }
    };
}

test_integer_conversions_and_operations!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

#[cfg(test)]
mod big_int_tests {
    use super::*;
    use alloc::vec::Vec;

    fn signed(value: &BigInt) -> Vec<u8> {
        <BigInt as IntegerType>::to_signed_bytes_be(value)
            .0
            .as_ref()
            .to_vec()
    }

    fn unsigned(value: &BigInt) -> Vec<u8> {
        <BigInt as IntegerType>::to_unsigned_bytes_be(value)
            .0
            .as_ref()
            .to_vec()
    }

    #[test]
    fn signed_octets_carry_the_sign_in_the_fewest_octets() {
        let positive = BigInt::from(1u128 << 71);
        let negative = -&positive;
        assert_eq!(signed(&positive), [0x00, 0x80, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(signed(&negative), [0x80, 0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            signed(&(&negative - 1)),
            [0xFF, 0x7F, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF]
        );
        assert_eq!(signed(&BigInt::ZERO), [0x00]);
    }

    #[test]
    fn unsigned_octets_are_the_magnitude_and_zero_for_a_negative_value() {
        assert_eq!(
            unsigned(&BigInt::from(1u128 << 71)),
            [0x80, 0, 0, 0, 0, 0, 0, 0, 0]
        );
        assert_eq!(unsigned(&BigInt::from(-5)), [0x00]);
        assert_eq!(unsigned(&BigInt::ZERO), [0x00]);
    }

    #[test]
    fn the_top_bit_is_a_sign_only_in_signed_octets() {
        let octets = [0x80, 0, 0, 0, 0, 0, 0, 0, 0];
        let codec = crate::Codec::Ber;
        assert_eq!(
            BigInt::try_from_bytes(&octets, codec).unwrap(),
            -BigInt::from(1u128 << 71)
        );
        assert_eq!(
            BigInt::try_from_unsigned_bytes(&octets, codec).unwrap(),
            BigInt::from(1u128 << 71)
        );
    }

    #[test]
    fn a_negative_magnitude_carries_across_octets() {
        let octets = [0xFF, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(
            BigInt::try_from_bytes(&octets, crate::Codec::Ber).unwrap(),
            -BigInt::from(1u128 << 64)
        );
    }

    #[test]
    fn integers_wider_than_the_stack_buffer_take_the_heap() {
        let wide = BigInt::from(1) << (STACK_INTEGER_OCTETS * 8 + 8);
        for value in [&wide, &-&wide] {
            let octets = value.to_signed_bytes_be();
            assert_eq!(signed(value), octets);
            assert_eq!(
                BigInt::try_from_bytes(&octets, crate::Codec::Ber).unwrap(),
                *value
            );
        }
        assert_eq!(unsigned(&wide), wide.magnitude().to_bytes_be());
        assert_eq!(
            BigInt::try_from_unsigned_bytes(&wide.magnitude().to_bytes_be(), crate::Codec::Ber)
                .unwrap(),
            wide
        );
    }
}
