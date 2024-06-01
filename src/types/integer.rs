use crate::types::{constraints, AsnType, Constraints, Extensible};
use crate::Tag;
use num_bigint::{BigInt, BigUint};
use num_traits::{Pow, PrimInt, Signed, ToBytes, ToPrimitive, Zero};

#[cfg(any(target_pointer_width = "16", target_pointer_width = "32"))]
compile_error!("rasn requires currently a 64-bit pointer width.");
#[cfg(not(target_has_atomic = "128"))]
compile_error!("rasn requires currently support for 128-bit integers (i128).");

// It seems that just ~1% performance difference between i64 and i128 (at least on M2 Mac)
// If disabled, it should be meant for targets which cannot use i128
// TODO: currently, library is not completely i128-free

// Currently available for small performance improvements, but maybe useful in future when i128 dropped completely
// for some specific targets
#[cfg(not(feature = "i128"))]
pub type PrimitiveInteger = i64;

#[cfg(all(target_pointer_width = "64", feature = "i128"))]
pub type PrimitiveInteger = i128;
pub const PRIMITIVE_BYTE_WIDTH: usize = core::mem::size_of::<PrimitiveInteger>();

macro_rules! impl_from_prim_ints {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Integer {
                fn from(value: $t) -> Self {
                    let primitive = PrimitiveInteger::try_from(value);
                    match primitive {
                        Ok(value) => Integer::Primitive(value),
                        Err(_) => Integer::Big(BigInt::from(value)),
                    }
                }
            }
            impl From<&$t> for Integer {
                fn from(value: &$t) -> Self {
                    let primitive = PrimitiveInteger::try_from(*value);
                    match primitive {
                        Ok(value) => Integer::Primitive(value),
                        Err(_) => Integer::Big(BigInt::from(*value)),
                    }
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
                    Integer::Primitive(value) => $to_ty(value).ok_or(TryFromIntegerError::new(())),
                    Integer::Big(value) => {
                        value.try_into().map_err(|_| TryFromIntegerError::new(()))
                    }
                }
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
#[cfg(target_pointer_width = "64")]
impl_try_from_bigint!(u64, ToPrimitive::to_u64);
impl_try_from_bigint!(usize, ToPrimitive::to_usize);

impl_try_from_bigint!(i8, ToPrimitive::to_i8);
impl_try_from_bigint!(i16, ToPrimitive::to_i16);
impl_try_from_bigint!(i32, ToPrimitive::to_i32);
#[cfg(target_pointer_width = "64")]
impl_try_from_bigint!(i64, ToPrimitive::to_i64);
impl_try_from_bigint!(isize, ToPrimitive::to_isize);

#[cfg(all(target_has_atomic = "128", feature = "i128"))]
impl_try_from_bigint!(u128, ToPrimitive::to_u128);
#[cfg(all(target_has_atomic = "128", feature = "i128"))]
impl_try_from_bigint!(i128, ToPrimitive::to_i128);

impl From<BigInt> for Integer {
    fn from(value: BigInt) -> Self {
        Integer::Big(value)
    }
}

#[cfg(all(target_has_atomic = "128", feature = "i128"))]
impl From<u128> for Integer {
    fn from(value: u128) -> Self {
        Integer::Big(value.into())
    }
}
// Note: usize and u64 will be converted to BigInt when `PrimitiveInteger` is i64
impl_from_prim_ints! {
    i8,
    i16,
    i32,
    isize,
    i64,
    u8,
    u16,
    u32,
    u64,
    usize
}
#[cfg(all(target_has_atomic = "128", feature = "i128"))]
impl_from_prim_ints! {
    i128
}

/// Fixed-size byte presentations for signed primitive integers
pub trait PrimIntBytes: PrimInt + Signed + ToBytes {
    /// Minimal number of bytes needed to present unsigned integer
    fn unsigned_bytes_needed(&self) -> usize {
        if self.is_zero() {
            1
        } else {
            PRIMITIVE_BYTE_WIDTH - (self.leading_zeros() / 8) as usize
        }
    }

    /// Minimal number of bytes needed to present signed integer
    fn signed_bytes_needed(&self) -> usize {
        // if self.is_negative() {
        //     let leading_ones = self.leading_ones() as usize;
        //     if leading_ones % 8 == 0 {
        //         PRIMITIVE_BYTE_WIDTH - leading_ones / 8 + 1
        //     } else {
        //         PRIMITIVE_BYTE_WIDTH - leading_ones / 8
        //     }
        // } else {
        //     let leading_zeroes = self.leading_zeros() as usize;
        //     if leading_zeroes % 8 == 0 {
        //         PRIMITIVE_BYTE_WIDTH - leading_zeroes / 8 + 1
        //     } else {
        //         PRIMITIVE_BYTE_WIDTH - leading_zeroes / 8
        //     }
        // }
        let leading_bits = if self.is_negative() {
            self.leading_ones() as usize
        } else {
            self.leading_zeros() as usize
        };

        let full_bytes = PRIMITIVE_BYTE_WIDTH - leading_bits / 8;
        let extra_byte = (leading_bits % 8 == 0) as usize;
        full_bytes + extra_byte
    }

    /// Calculate minimal number of bytes to show integer based on `signed` status
    /// Returns slice an with fixed-width `N` and number of needed bytes
    /// We need only some of the bytes, more optimal than just using `to_be_bytes` we would need to drop the rest anyway
    fn needed_as_be_bytes<const N: usize>(&self, signed: bool) -> ([u8; N], usize) {
        let bytes: [u8; N] = self.to_le_bytes().as_ref().try_into().unwrap(); // unwrap_or([0; N]);
        let needed = if signed {
            self.signed_bytes_needed()
        } else {
            self.unsigned_bytes_needed()
        };
        let mut slice_reversed: [u8; N] = [0; N];
        let len = needed;
        for i in 0..len {
            slice_reversed[i] = bytes[len - 1 - i];
        }
        (slice_reversed, needed)
    }
    /// Swap bytes order and copy to new `N` sized slice
    /// Returns also the count of "needed" bytes, since the slice width might be much larger
    fn min_swap_bytes<const N: usize>(bytes: &[u8]) -> ([u8; N], usize) {
        let mut slice_reversed: [u8; N] = [0; N];
        for i in 0..bytes.len() {
            slice_reversed[i] = bytes[bytes.len() - 1 - i];
        }
        (slice_reversed, bytes.len())
    }
    /// Swap bytes order and copy to new `N` sized slice
    fn swap_to_be_bytes<const N: usize>(bytes: &[u8]) -> ([u8; N], usize) {
        #[cfg(target_endian = "little")]
        {
            Self::min_swap_bytes(bytes)
        }
        #[cfg(target_endian = "big")]
        {
            (bytes.try_into().unwrap_or([0; N]), bytes.len())
        }
    }
}
impl PrimIntBytes for PrimitiveInteger {}

/// `Integer`` type is enum wrapper for `PrimitiveInteger` and `BigInt`
/// `PrimitiveInteger` is `i128` by default when `i128` feature is enabled, otherwise `i64`.
/// Combination of both types is used to optimize memory usage and performance, while also allowing arbitrary large numbers.
#[derive(Debug, Clone, Ord, Hash, Eq, PartialEq, PartialOrd)]
#[non_exhaustive]
pub enum Integer {
    Primitive(PrimitiveInteger),
    Big(BigInt),
}
impl Integer {
    /// Returns generic `PrimitiveInteger` wrapper type if that is the current variant
    #[must_use]
    pub fn as_primitive(&self) -> Option<&PrimitiveInteger> {
        match self {
            Integer::Primitive(ref inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns inner heap-allocated `BigInt` if that is the current variant
    #[must_use]
    pub fn as_big(&self) -> Option<&BigInt> {
        match self {
            Integer::Big(ref inner) => Some(inner),
            _ => None,
        }
    }
    /// Return Big-Endian presentation of `Integer`
    /// Will always create new heap allocation, use carefully
    #[must_use]
    #[inline]
    pub fn to_be_bytes(&self) -> alloc::vec::Vec<u8> {
        match self {
            Integer::Primitive(value) => value.to_be_bytes().to_vec(),
            Integer::Big(value) => value.to_signed_bytes_be(),
        }
    }

    /// Return `Integer` as unsigned bytes if possible
    /// Typically this means that resulting byte array does not include any padded zeros or 0xFF
    /// Will always create new heap allocation, use carefully
    /// Rather, if possible, match for underlying primitive integer type to get raw slice without conversions
    #[must_use]
    #[inline]
    pub fn to_unsigned_be_bytes(&self) -> Option<alloc::vec::Vec<u8>> {
        match self {
            Integer::Primitive(value) => {
                let (value, len) = value.needed_as_be_bytes::<PRIMITIVE_BYTE_WIDTH>(false);
                Some(value[..len].to_vec())
            }
            Integer::Big(value) => Some(value.to_biguint()?.to_bytes_be()),
        }
    }

    /// New `Integer` from signed bytes.
    /// Inner type is defined based on the amount of bytes.
    /// `Integer::Primitive` if less or equal defined primitive width `PRIMITIVE_BYTE_WIDTH`, otherwise `Integer::Big`.
    #[must_use]
    pub fn from_signed_be_bytes(bytes: &[u8]) -> Self {
        let mut array = [0u8; PRIMITIVE_BYTE_WIDTH];
        if bytes.len() <= PRIMITIVE_BYTE_WIDTH {
            let pad = if bytes[0] & 0x80 == 0 { 0 } else { 0xff };

            array[..PRIMITIVE_BYTE_WIDTH - bytes.len()].fill(pad);
            array[PRIMITIVE_BYTE_WIDTH - bytes.len()..].copy_from_slice(bytes);

            Integer::Primitive(PrimitiveInteger::from_be_bytes(array))
        } else {
            // If the byte slice is longer than the byte width, treat it as a BigInt
            Integer::Big(BigInt::from_signed_bytes_be(bytes))
        }
    }
    /// New `Integer` from unsigned bytes.
    /// Inner type is defined based on the amount of bytes.
    /// `Integer::Primitive` if less or equal defined primitive width `PRIMITIVE_BYTE_WIDTH`, otherwise `Integer::Big`.
    #[must_use]
    pub fn from_be_bytes(bytes: &[u8]) -> Self {
        if bytes.len() <= PRIMITIVE_BYTE_WIDTH {
            let mut array = [0u8; PRIMITIVE_BYTE_WIDTH];
            for i in 0..bytes.len() {
                array[PRIMITIVE_BYTE_WIDTH - bytes.len() + i] = bytes[i];
            }
            Integer::Primitive(PrimitiveInteger::from_be_bytes(array))
        } else {
            Integer::Big(BigUint::from_bytes_be(bytes).into())
        }
    }
    /// Returns the amount of bytes that integer **might** take to present
    /// Useful on pre-allocating vectors with minimum size
    /// *Not* free operation
    #[must_use]
    pub fn byte_length(&self) -> usize {
        match self {
            Integer::Primitive(value) => value.to_ne_bytes().len(),
            Integer::Big(value) => value.to_ne_bytes().len(),
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
    /// Returns the sign of the number.
    #[must_use]
    pub fn signum(&self) -> Self {
        match self {
            Integer::Primitive(value) => Integer::Primitive(PrimitiveInteger::from(value.signum())),
            Integer::Big(value) => Integer::Big(value.signum()),
        }
    }
    /// Whether the number is positive.
    #[must_use]
    pub fn is_positive(&self) -> bool {
        match self {
            Integer::Primitive(value) => value.is_positive(),
            Integer::Big(value) => value.is_positive(),
        }
    }
    /// Whether the number is negative.
    #[must_use]
    pub fn is_negative(&self) -> bool {
        match self {
            Integer::Primitive(value) => value.is_negative(),
            Integer::Big(value) => value.is_negative(),
        }
    }
    /// Whether the number is zero.
    #[must_use]
    pub fn is_zero(&self) -> bool {
        match self {
            Integer::Primitive(value) => value.is_zero(),
            Integer::Big(value) => value.is_zero(),
        }
    }
}

impl core::ops::Sub for Integer {
    type Output = Self;

    /// Subtraction of two `Integer` values
    /// Will automatically convert to `BigInt` internally if the result is too large for `PrimitiveInteger`
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Integer::Primitive(lhs), Integer::Primitive(rhs)) => {
                let result = lhs.checked_sub(rhs);
                if let Some(result) = result {
                    Integer::Primitive(result)
                } else {
                    Integer::Big(BigInt::from(lhs) - rhs)
                }
            }
            (Integer::Big(lhs), Integer::Big(rhs)) => Integer::Big(lhs - rhs),
            (Integer::Primitive(lhs), Integer::Big(rhs)) => Integer::Big(lhs - rhs),
            (Integer::Big(lhs), Integer::Primitive(rhs)) => Integer::Big(lhs - rhs),
        }
    }
}
impl core::ops::Add for Integer {
    type Output = Self;

    /// Addition of two `Integer` values
    /// Will automatically convert to `BigInt` internally if the result is too large for `PrimitiveInteger`
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Integer::Primitive(lhs), Integer::Primitive(rhs)) => {
                let result = lhs.checked_add(rhs);
                if let Some(result) = result {
                    Integer::Primitive(result)
                } else {
                    Integer::Big(BigInt::from(lhs) + rhs)
                }
            }
            (Integer::Big(lhs), Integer::Big(rhs)) => Integer::Big(lhs + rhs),
            (Integer::Primitive(lhs), Integer::Big(rhs)) => Integer::Big(lhs + rhs),
            (Integer::Big(lhs), Integer::Primitive(rhs)) => Integer::Big(lhs + rhs),
        }
    }
}

impl core::ops::Mul for Integer {
    type Output = Self;
    /// Multiplication of two `Integer` values
    /// Will automatically convert to `BigInt` internally if the result is too large for `PrimitiveInteger`
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Integer::Primitive(lhs), Integer::Primitive(rhs)) => {
                let result = lhs.checked_mul(rhs);
                if let Some(result) = result {
                    Integer::Primitive(result)
                } else {
                    Integer::Big(BigInt::from(lhs) * rhs)
                }
            }
            (Integer::Big(lhs), Integer::Big(rhs)) => Integer::Big(lhs * rhs),
            (Integer::Primitive(lhs), Integer::Big(rhs)) => Integer::Big(lhs * rhs),
            (Integer::Big(lhs), Integer::Primitive(rhs)) => Integer::Big(lhs * rhs),
        }
    }
}

impl Pow<Integer> for Integer {
    type Output = Self;

    /// Raises `Integer` to the power of `Integer`, using exponentiation by squaring.
    /// Will automatically convert to `BigInt` internally if the result is too large for `PrimitiveInteger`
    /// Will silently truncate too large exponents (> u32::MAX)
    fn pow(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Integer::Primitive(lhs), Integer::Primitive(rhs)) => {
                let result = lhs.checked_pow(rhs as u32);
                if let Some(result) = result {
                    Integer::Primitive(result)
                } else {
                    Integer::Big(BigInt::from(lhs).pow(rhs as u32))
                }
            }
            (Integer::Big(lhs), Integer::Big(rhs)) => Integer::Big(lhs.pow(rhs.to_u32().unwrap())),
            (Integer::Primitive(lhs), Integer::Big(rhs)) => {
                Integer::Big(BigInt::from(lhs).pow(rhs.to_u32().unwrap()))
            }
            (Integer::Big(lhs), Integer::Primitive(rhs)) => Integer::Big(lhs.pow(rhs as u128)),
        }
    }
}

impl alloc::fmt::Display for Integer {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        match self {
            Integer::Primitive(value) => write!(f, "{}", *value),
            Integer::Big(value) => write!(f, "{value}"),
        }
    }
}

impl core::default::Default for Integer {
    fn default() -> Self {
        Integer::Primitive(PrimitiveInteger::default())
    }
}

/// An integer which has encoded constraint range between `START` and `END`.
#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ConstrainedInteger<const START: PrimitiveInteger, const END: PrimitiveInteger>(
    pub(crate) Integer,
);

impl<const START: PrimitiveInteger, const END: PrimitiveInteger> AsnType
    for ConstrainedInteger<START, END>
{
    const TAG: Tag = Tag::INTEGER;
    const CONSTRAINTS: Constraints<'static> =
        Constraints::new(&[constraints::Constraint::Value(Extensible::new(
            constraints::Value::new(constraints::Bounded::const_new(START, END)),
        ))]);
}

impl<const START: PrimitiveInteger, const END: PrimitiveInteger> core::ops::Deref
    for ConstrainedInteger<START, END>
{
    type Target = Integer;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Into<Integer>, const START: PrimitiveInteger, const END: PrimitiveInteger> From<T>
    for ConstrainedInteger<START, END>
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}
