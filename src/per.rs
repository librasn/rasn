pub mod de;
pub mod enc;

use crate::types::Constraints;
// The derives for the X.691 clause 32 encoding types below need these in scope.
use crate::{AsnType as _, Decoder as _};

pub use self::{de::Decoder, enc::Encoder};

const SIXTEEN_K: u16 = 16384;
const THIRTY_TWO_K: u16 = 32768;
const FOURTY_EIGHT_K: u16 = 49152;
const SIXTY_FOUR_K: u32 = 65536;
const SMALL_UNSIGNED_CONSTRAINT: Constraints = constraints!(value_constraint!(0, 63));
const LARGE_UNSIGNED_CONSTRAINT: Constraints = constraints!(value_constraint!(start: 0));

/// ITU-T X.691 (02/2021) §32.2.7: `DATE` has the property settings
/// "Basic=Date Date=YMD Year=Basic", so PER encodes it as if it were this
/// `DATE-ENCODING` type, defined in an AUTOMATIC TAGS environment.
#[derive(crate::AsnType, crate::Encode, crate::Decode, Debug, Clone, Copy, PartialEq)]
#[rasn(crate_root = "crate", automatic_tags)]
struct DateEncoding {
    year: YearEncoding,
    #[rasn(value("1..=12"))]
    month: u8,
    #[rasn(value("1..=31"))]
    day: u8,
}

/// ITU-T X.691 (02/2021) §32.2.3: `YEAR-ENCODING`, which gives common years a
/// six-bit or ten-bit encoding.
#[derive(crate::AsnType, crate::Encode, crate::Decode, Debug, Clone, Copy, PartialEq)]
#[rasn(crate_root = "crate", choice, automatic_tags)]
enum YearEncoding {
    Immediate(ImmediateYear),
    NearFuture(NearFutureYear),
    NearPast(NearPastYear),
    /// `INTEGER (MIN..1748 | 2277..MAX)`, which is not a PER-visible range and
    /// so is encoded as an unconstrained integer.
    Remainder(i32),
}

#[derive(crate::AsnType, crate::Encode, crate::Decode, Debug, Clone, Copy, PartialEq)]
#[rasn(crate_root = "crate", delegate, value("2005..=2020"))]
struct ImmediateYear(u16);

#[derive(crate::AsnType, crate::Encode, crate::Decode, Debug, Clone, Copy, PartialEq)]
#[rasn(crate_root = "crate", delegate, value("2021..=2276"))]
struct NearFutureYear(u16);

#[derive(crate::AsnType, crate::Encode, crate::Decode, Debug, Clone, Copy, PartialEq)]
#[rasn(crate_root = "crate", delegate, value("1749..=2004"))]
struct NearPastYear(u16);

impl From<crate::types::Date> for DateEncoding {
    fn from(date: crate::types::Date) -> Self {
        use chrono::Datelike;
        let year = date.year();
        Self {
            year: match year {
                2005..=2020 => YearEncoding::Immediate(ImmediateYear(year as u16)),
                2021..=2276 => YearEncoding::NearFuture(NearFutureYear(year as u16)),
                1749..=2004 => YearEncoding::NearPast(NearPastYear(year as u16)),
                _ => YearEncoding::Remainder(year),
            },
            month: date.month() as u8,
            day: date.day() as u8,
        }
    }
}

impl DateEncoding {
    /// The date, unless the components do not form one (such as 30 February).
    fn into_date(self) -> Option<crate::types::Date> {
        let year = match self.year {
            YearEncoding::Immediate(ImmediateYear(year))
            | YearEncoding::NearFuture(NearFutureYear(year))
            | YearEncoding::NearPast(NearPastYear(year)) => i32::from(year),
            YearEncoding::Remainder(year) => year,
        };
        crate::types::Date::from_ymd_opt(year, self.month.into(), self.day.into())
    }
}

/// Attempts to decode `T` from `input` using PER.
pub(crate) fn decode<T: crate::Decode>(
    options: de::DecoderOptions,
    input: &[u8],
) -> Result<T, crate::error::DecodeError> {
    T::decode(&mut crate::per::de::Decoder::<0, 0>::new(
        crate::types::BitStr::from_slice(input),
        options,
    ))
}
/// Attempts to decode `T` from `input` using PER. Returns both `T` and reference to the remainder of the input.
///
/// # Errors
/// Returns `DecodeError` if `input` is not valid PER encoding specific to the expected type.
pub(crate) fn decode_with_remainder<T: crate::Decode>(
    options: de::DecoderOptions,
    input: &[u8],
) -> Result<(T, &[u8]), crate::error::DecodeError> {
    let decoder = &mut Decoder::<0, 0>::new(crate::types::BitStr::from_slice(input), options);
    let decoded_instance = T::decode(decoder)?;
    let remaining_bits = decoder.input().len();
    // Consider only whole bytes, ignore padding bits
    let remaining_size = remaining_bits / 8;
    debug_assert!(input.len() >= remaining_size);
    Ok((decoded_instance, &input[input.len() - remaining_size..]))
}

/// Attempts to encode `value` to PER.
pub(crate) fn encode<T: crate::Encode>(
    options: enc::EncoderOptions,
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
    let mut enc = crate::per::enc::Encoder::<0, 0>::new(options);

    value.encode(&mut enc)?;

    Ok(enc.output_into_vec())
}

/// Encodes `value` to PER into an existing `buffer`, reusing its allocation.
/// The buffer is cleared before encoding.
pub(crate) fn encode_buf<T: crate::Encode>(
    options: enc::EncoderOptions,
    value: &T,
    buffer: &mut alloc::vec::Vec<u8>,
) -> Result<(), crate::error::EncodeError> {
    let raw = core::mem::take(buffer);
    let output = crate::types::BitString::from_vec(raw);
    let mut enc = crate::per::enc::Encoder::<0, 0>::new_with_output(options, output);
    value.encode(&mut enc)?;
    *buffer = enc.output_into_vec();
    Ok(())
}

/// Attempts to decode `T` from `input` using PER.
pub(crate) fn decode_with_constraints<T: crate::Decode>(
    options: de::DecoderOptions,
    constraints: Constraints,
    input: &[u8],
) -> Result<T, crate::error::DecodeError> {
    T::decode_with_constraints(
        &mut crate::per::de::Decoder::<0, 0>::new(crate::types::BitStr::from_slice(input), options),
        constraints,
    )
}

/// Attempts to encode `value` to PER.
pub(crate) fn encode_with_constraints<T: crate::Encode>(
    options: enc::EncoderOptions,
    constraints: Constraints,
    value: &T,
) -> Result<alloc::vec::Vec<u8>, crate::error::EncodeError> {
    let mut enc = crate::per::enc::Encoder::<0, 0>::new(options);

    value.encode_with_constraints(&mut enc, constraints)?;

    Ok(enc.output_into_vec())
}
