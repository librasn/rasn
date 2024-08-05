use crate::types::{constraints, AsnType, Constraints, Extensible};
use crate::Tag;
use num_bigint::{BigInt, BigUint, ToBigInt};
use num_traits::{identities::Zero, Signed, ToBytes};

/// `Integer`` type is newtype wrapper for any integer type which implements `IntegerType` trait. Uses `i128` by default if no type defined.
#[derive(Debug, Clone, Ord, Hash, Eq, PartialEq, PartialOrd)]
#[non_exhaustive]
pub struct Integer<I: IntegerType = i128>(pub I);

// impl<I: IntegerType> Integer<I> {
//     /// Create a new `Integer` from the given value.
//     pub fn new(value: I) -> Self {
//         Self(value)
//     }
// }

impl<I: IntegerType> core::ops::Deref for Integer<I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I: IntegerType> Default for Integer<I> {
    fn default() -> Self {
        Self(I::default())
    }
}

impl<I: IntegerType> core::fmt::Display for Integer<I> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<I: IntegerType> core::ops::DerefMut for Integer<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl<I: IntegerType> core::ops::Add for Integer<I> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

macro_rules! impl_add_for_integer {
    ($($t:ty),*) => {
        $(
            impl<I: IntegerType + core::ops::Add<$t, Output = I>> core::ops::Add<$t> for Integer<I> {
                type Output = Self;

                fn add(self, rhs: $t) -> Self::Output {
                    Self(self.0 + rhs)
                }
            }

            impl<I: IntegerType + From<$t> + core::ops::Add<I, Output = I>> core::ops::Add<Integer<I>> for $t {
                type Output = Integer<I>;

                fn add(self, rhs: Integer<I>) -> Self::Output {
                    Integer(I::from(self) + rhs.0)
                }
            }
        )*
    };
}

impl_add_for_integer!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize, BigInt);

macro_rules! impl_sub_for_integer {
    ($($t:ty),*) => {
        $(
            impl<I: IntegerType + core::ops::Sub<$t, Output = I>> core::ops::Sub<$t> for Integer<I> {
                type Output = Self;

                fn sub(self, rhs: $t) -> Self::Output {
                    Self(self.0 - rhs)
                }
            }

            impl<I: IntegerType + From<$t> + core::ops::Sub<I, Output = I>> core::ops::Sub<Integer<I>> for $t {
                type Output = Integer<I>;

                fn sub(self, rhs: Integer<I>) -> Self::Output {
                    Integer(I::from(self) - rhs.0)
                }
            }
        )*
    };
}
impl_sub_for_integer!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, usize, isize, BigInt);

macro_rules! impl_from_integer {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Integer<$t> {
                fn from(value: $t) -> Self {
                    Self(value)
                }
            }
        )*
    };
}
macro_rules! impl_from_big_integer {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Integer<BigInt> {
                fn from(value: $t) -> Self {
                    Self(BigInt::from(value))
                }
            }
        )*
    };
}
macro_rules! impl_from_i128_integer {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Integer<i128> {
                fn from(value: $t) -> Self {
                    Self(value as i128)
                }
            }
        )*
    };
}

impl_from_integer!(BigInt, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
// TODO u128 is truncated
impl_from_i128_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, isize);
impl_from_big_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct TryFromIntegerError<T: IntegerType> {
    original: T,
}

impl<T: IntegerType> TryFromIntegerError<T> {
    fn new(original: T) -> Self {
        TryFromIntegerError { original }
    }
    fn __description(&self) -> &str {
        "out of range conversion regarding integer conversion attempted"
    }
    pub fn into_original(self) -> T {
        self.original
    }
}

impl<T: IntegerType> alloc::fmt::Display for TryFromIntegerError<T> {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        self.__description().fmt(f)
    }
}
macro_rules! impl_try_into_integer {
    ($($t:ty),*) => {
        $(
            impl<I : IntegerType> core::convert::TryFrom<Integer<I>> for $t {
                type Error = TryFromIntegerError<I>;
                fn try_from(value: Integer<I>) -> Result<Self, Self::Error> {
                    value.0.clone().try_into().map_err(|_| TryFromIntegerError::new(value.0))
                }
            }
            impl<I : IntegerType> core::convert::TryFrom<&Integer<I>> for $t {
                type Error = TryFromIntegerError<I>;
                fn try_from(value: &Integer<I>) -> Result<Self, Self::Error> {
                    value.0.clone().try_into().map_err(|_| TryFromIntegerError::new(value.0.clone()))
                }
            }
        )*
    };
}

impl_try_into_integer!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

/// An integer which has encoded constraint range between `START` and `END`.
#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct ConstrainedInteger<const START: i128, const END: i128, I: IntegerType = i128>(
    pub(crate) Integer<I>,
);

impl<I: IntegerType, const START: i128, const END: i128> AsnType
    for ConstrainedInteger<START, END, I>
{
    const TAG: Tag = Tag::INTEGER;
    const CONSTRAINTS: Constraints<'static> =
        Constraints::new(&[constraints::Constraint::Value(Extensible::new(
            constraints::Value::new(constraints::Bounded::const_new(START, END)),
        ))]);
}

impl<I: IntegerType, const START: i128, const END: i128> core::ops::Deref
    for ConstrainedInteger<START, END, I>
{
    type Target = Integer<I>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<I: IntegerType, T: Into<Integer<I>>, const START: i128, const END: i128> From<T>
    for ConstrainedInteger<START, END, I>
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

pub trait IntegerType:
    Sized
    + Clone
    + core::fmt::Debug
    + core::fmt::Display
    + Default
    + TryInto<i8>
    + TryInto<i16>
    + TryInto<i32>
    + TryInto<i64>
    + TryInto<i128>
    + TryInto<isize>
    + TryFrom<i8>
    + TryFrom<i16>
    + TryFrom<i32>
    + TryFrom<i64>
    + TryFrom<i128>
    + TryFrom<isize>
    + TryFrom<BigInt>
    + TryInto<u8>
    + TryInto<u16>
    + TryInto<u32>
    + TryInto<u64>
    + TryInto<u128>
    + TryInto<usize>
    + TryFrom<u8>
    + TryFrom<u16>
    + TryFrom<u32>
    + TryFrom<u64>
    + TryFrom<u128>
    + TryFrom<usize>
    + Into<BigInt>
    + ToBigInt
    + num_traits::Num
    + num_traits::CheckedAdd
    + num_traits::ToPrimitive
{
    const WIDTH: u32;
    const BYTE_WIDTH: usize = Self::WIDTH as usize / 8;

    fn try_from_bytes(input: &[u8], codec: crate::Codec)
        -> Result<Self, crate::error::DecodeError>;

    fn try_from_signed_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError>;

    fn try_from_unsigned_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError>;

    /// Finds the minimum number of bytes needed to present the unsigned integer. (drops leading zeros or ones)
    fn unsigned_bytes_needed(&self) -> usize;

    /// Finds the minimum number of bytes needed to present the signed integer. (drops leading zeros or ones)
    fn signed_bytes_needed(&self) -> usize;

    /// Returns minimum number defined by `usize` of signed Big-endian bytes needed to encode the integer.
    fn to_signed_bytes_be(&self) -> (impl AsRef<[u8]>, usize);

    /// Returns minimum number defined by `usize` of unsigned Big-endian bytes needed to encode the integer.
    fn to_unsigned_bytes_be(&self) -> (impl AsRef<[u8]>, usize);

    // `num_traits::WrappingAdd` is not implemented for `BigInt`
    #[doc(hidden)]
    fn wrapping_add(self, other: Self) -> Self;

    fn is_negative(&self) -> bool;
}

/// Encode the given `N` sized integer as big-endian bytes and determine the number of bytes needed.
/// Needed bytes drops unnecessary leading zeros or ones.
fn needed_as_be_bytes<T: ToBytes + IntegerType, const N: usize>(
    value: T,
    signed: bool,
) -> ([u8; N], usize) {
    let bytes: [u8; N] = value.to_le_bytes().as_ref().try_into().unwrap_or([0; N]);
    let needed = if signed {
        value.signed_bytes_needed()
    } else {
        value.unsigned_bytes_needed()
    };
    let mut slice_reversed: [u8; N] = [0; N];
    // About 2.5x speed when compared to `copy_from_slice` and `.reverse()`
    for i in 0..needed {
        slice_reversed[i] = bytes[needed - 1 - i];
    }
    (slice_reversed, needed)
}
macro_rules! integer_type_impl {
    ((signed $t1:ty, $t2:ty), $($ts:tt)*) => {
        impl IntegerType for $t1 {
            const WIDTH: u32 = <$t1>::BITS;

            fn try_from_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                Self::try_from_signed_bytes(input, codec)
            }

            fn try_from_signed_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                const BYTE_SIZE: usize = (<$t1>::BITS / 8) as usize;
                if input.is_empty() {
                    return Err(crate::error::DecodeError::unexpected_empty_input(codec));
                }

                // in the case of superfluous leading bytes (especially zeroes),
                // we may still want to try to decode the integer even though
                // the length is >BYTE_SIZE ...
                let leading_byte = if input[0] & 0x80 == 0x80 { 0xFF } else { 0x00 };
                let input_iter = input.iter().copied().skip_while(|n| *n == leading_byte);
                let data_length = input_iter.clone().count();

                // ... but if its still too large after skipping leading bytes,
                // there's no way to decode this without overflowing
                if data_length > BYTE_SIZE {
                    return Err(crate::error::DecodeError::integer_overflow(<$t1>::BITS, codec));
                }

                let mut bytes = [leading_byte; BYTE_SIZE];
                let start = bytes.len() - data_length;

                for (b, d) in bytes[start..].iter_mut().zip(input_iter) {
                    *b = d;
                }

                Ok(Self::from_be_bytes(bytes))
            }

            fn try_from_unsigned_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                Ok(<$t2>::try_from_bytes(input, codec)? as $t1)
            }
            fn unsigned_bytes_needed(&self) -> usize {
                (*self as $t2).unsigned_bytes_needed()
            }
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

            fn to_signed_bytes_be(&self) -> (impl AsRef<[u8]>, usize) {
                const N: usize = core::mem::size_of::<$t1>();
                needed_as_be_bytes::<$t1, N>(*self, true)
            }
            fn to_unsigned_bytes_be(&self) -> (impl AsRef<[u8]>, usize) {
                const N: usize = core::mem::size_of::<$t2>();
                needed_as_be_bytes::<$t2, N>(*self as $t2, false)
            }

            fn wrapping_add(self, other: Self) -> Self {
                self.wrapping_add(other)
            }
            fn is_negative(&self) -> bool {
                <Self as Signed>::is_negative(self)
            }
        }

        integer_type_impl!($($ts)*);
    };
    ((unsigned $t1:ty, $t2:ty), $($ts:tt)*) => {



        impl IntegerType for $t1 {
            const WIDTH: u32 = <$t1>::BITS;

            fn try_from_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                Self::try_from_unsigned_bytes(input, codec)
            }

            fn try_from_signed_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                Ok(<$t2>::try_from_bytes(input, codec)? as $t1)
            }

            fn try_from_unsigned_bytes(
                input: &[u8],
                codec: crate::Codec,
            ) -> Result<Self, crate::error::DecodeError> {
                const BYTE_SIZE: usize = (<$t1>::BITS / 8) as usize;
                if input.is_empty() {
                    return Err(crate::error::DecodeError::unexpected_empty_input(codec));
                }

                let input_iter = input.iter().copied().skip_while(|n| *n == 0x00);
                let data_length = input_iter.clone().count();

                if data_length > BYTE_SIZE {
                    return Err(crate::error::DecodeError::integer_overflow(<$t1>::BITS, codec));
                }

                let mut bytes = [0x00; BYTE_SIZE];
                let start = bytes.len() - data_length;

                for (b, d) in bytes[start..].iter_mut().zip(input_iter) {
                    *b = d;
                }


                Ok(Self::from_be_bytes(bytes))
            }
            fn unsigned_bytes_needed(&self) -> usize {
                if self.is_zero() {
                    1
                } else {
                    let significant_bits = Self::WIDTH as usize - self.leading_zeros() as usize;
                    (significant_bits + 7) / 8
                }
            }
            fn signed_bytes_needed(&self) -> usize {
                (*self as $t2).signed_bytes_needed()
            }

            fn to_signed_bytes_be(&self) -> (impl AsRef<[u8]>, usize) {
                const N: usize = core::mem::size_of::<$t2>();
                needed_as_be_bytes::<$t2, N>(*self as $t2, true)
            }
            fn to_unsigned_bytes_be(&self) -> (impl AsRef<[u8]>, usize) {
                const N: usize = core::mem::size_of::<$t1>();
                needed_as_be_bytes::<$t1, N>(*self, false)
            }

            fn wrapping_add(self, other: Self) -> Self {
                self.wrapping_add(other)
            }
            fn is_negative(&self) -> bool {
                false
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

impl IntegerType for BigInt {
    const WIDTH: u32 = u32::MAX;

    fn try_from_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError> {
        if input.is_empty() {
            return Err(crate::error::DecodeError::unexpected_empty_input(codec));
        }

        Ok(BigInt::from_signed_bytes_be(input))
    }

    fn try_from_signed_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError> {
        Self::try_from_bytes(input, codec)
    }

    fn try_from_unsigned_bytes(
        input: &[u8],
        codec: crate::Codec,
    ) -> Result<Self, crate::error::DecodeError> {
        if input.is_empty() {
            return Err(crate::error::DecodeError::unexpected_empty_input(codec));
        }

        Ok(BigUint::from_bytes_be(input).into())
    }
    // Not needed for BigInt
    fn unsigned_bytes_needed(&self) -> usize {
        unreachable!()
    }

    // Not needed for BigInt
    fn signed_bytes_needed(&self) -> usize {
        unreachable!()
    }
    fn to_signed_bytes_be(&self) -> (impl AsRef<[u8]>, usize) {
        let bytes = self.to_signed_bytes_be();
        let len = bytes.len();
        (bytes, len)
    }
    fn to_unsigned_bytes_be(&self) -> (impl AsRef<[u8]>, usize) {
        let bytes = self.to_biguint().unwrap_or_default().to_bytes_be();
        let len = bytes.len();
        (bytes, len)
    }

    fn wrapping_add(self, other: Self) -> Self {
        self + other
    }
    fn is_negative(&self) -> bool {
        <Self as Signed>::is_negative(self)
    }
}
