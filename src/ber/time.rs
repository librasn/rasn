//! The character forms of the time types for every codec that carries time
//! values as text: GeneralizedTime and UTCTime as X.680 (§46, §47) defines
//! them, DATE as `YYYYMMDD` (X.690 §8.26.2), and the canonical forms that
//! X.690 requires of CER and DER (§11.7, §11.8).

use alloc::vec::Vec;

use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, NaiveTime, Timelike, Utc};

const NANOSECONDS_PER_SECOND: u64 = 1_000_000_000;

/// Parses a GeneralizedTime in any form X.680 §46.3 allows: the date and
/// hour, optional minutes and seconds, an optional fraction of the smallest
/// unit given after `.` or `,`, and either `Z`, a `±HHMM` offset or nothing,
/// which is read as UTC.
pub(crate) fn parse_generalized_time(text: &[u8]) -> Option<DateTime<FixedOffset>> {
    parse_generalized_time_form(text, false)
}

/// Parses a GeneralizedTime in the canonical form of X.690 §11.7: the time
/// to the second, a fraction of a second only after `.`, and `Z`.
pub(crate) fn parse_canonical_generalized_time(text: &[u8]) -> Option<DateTime<FixedOffset>> {
    parse_generalized_time_form(text, true)
}

fn parse_generalized_time_form(text: &[u8], canonical: bool) -> Option<DateTime<FixedOffset>> {
    let mut text = Characters(text);
    let date = text.date()?;
    let hour = text.number(2)?;
    let minute = text.optional_number(2);
    let second = minute.and_then(|_| text.optional_number(2));
    if canonical && second.is_none() {
        return None;
    }
    // The fraction is of the smallest unit given.
    let unit_seconds = match (minute, second) {
        (_, Some(_)) => 1,
        (Some(_), None) => 60,
        (None, None) => 3600,
    };
    let fraction = match text.separator(canonical) {
        Some(()) => text.fraction()? * unit_seconds,
        None => 0,
    };
    let zone = match text.zone(canonical)? {
        Some(offset) => offset,
        None if canonical => return None,
        None => FixedOffset::east_opt(0)?,
    };
    if !text.is_empty() {
        return None;
    }
    let time = time_of_day(hour, minute.unwrap_or(0), second.unwrap_or(0), fraction)?;
    date.and_time(time).and_local_timezone(zone).single()
}

/// Parses a UTCTime in any form X.680 §47.3 allows: a two-digit year, the
/// time to the minute or the second, and either `Z` or a `±HHMM` offset.
pub(crate) fn parse_utc_time(text: &[u8]) -> Option<DateTime<Utc>> {
    parse_utc_time_form(text, false)
}

/// Parses a UTCTime in the canonical form of X.690 §11.8: the time to the
/// second and `Z`.
pub(crate) fn parse_canonical_utc_time(text: &[u8]) -> Option<DateTime<Utc>> {
    parse_utc_time_form(text, true)
}

fn parse_utc_time_form(text: &[u8], canonical: bool) -> Option<DateTime<Utc>> {
    let mut text = Characters(text);
    // The century is not given; years from 70 are in the twentieth.
    let year = match text.number(2)? {
        year @ 70.. => 1900 + year,
        year => 2000 + year,
    };
    let month = text.number(2)?;
    let day = text.number(2)?;
    let hour = text.number(2)?;
    let minute = text.number(2)?;
    let second = text.optional_number(2);
    if canonical && second.is_none() {
        return None;
    }
    let zone = text.zone(canonical)??;
    if !text.is_empty() {
        return None;
    }
    let date = NaiveDate::from_ymd_opt(year as i32, month, day)?;
    let time = time_of_day(hour, minute, second.unwrap_or(0), 0)?;
    Some(
        date.and_time(time)
            .and_local_timezone(zone)
            .single()?
            .to_utc(),
    )
}

/// Parses a DATE, `YYYYMMDD`.
pub(crate) fn parse_date(text: &[u8]) -> Option<NaiveDate> {
    let mut text = Characters(text);
    let date = text.date()?;
    text.is_empty().then_some(date)
}

/// The time of day at `hour`, `minute` and `second`, plus `nanoseconds`.
/// A `second` of 60 is a leap second, which `chrono` represents as second
/// 59 with a billion or more nanoseconds.
fn time_of_day(hour: u32, minute: u32, second: u32, nanoseconds: u64) -> Option<NaiveTime> {
    if minute >= 60 {
        return None;
    }
    let (second, leap) = match second {
        60 => (59, NANOSECONDS_PER_SECOND),
        second @ 0..60 => (second, 0),
        _ => return None,
    };
    let seconds = u64::from(hour) * 3600
        + u64::from(minute) * 60
        + u64::from(second)
        + nanoseconds / NANOSECONDS_PER_SECOND;
    let nanoseconds = nanoseconds % NANOSECONDS_PER_SECOND + leap;
    NaiveTime::from_num_seconds_from_midnight_opt(
        u32::try_from(seconds).ok()?,
        u32::try_from(nanoseconds).ok()?,
    )
}

/// The characters of a time value that have not been parsed.
struct Characters<'a>(&'a [u8]);

impl Characters<'_> {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Parses a number of exactly `digits` decimal digits.
    fn number(&mut self, digits: usize) -> Option<u32> {
        let (number, rest) = self.0.split_at_checked(digits)?;
        let mut value = 0u32;
        for &digit in number {
            value = value * 10 + u32::from(digit.checked_sub(b'0').filter(|digit| *digit < 10)?);
        }
        self.0 = rest;
        Some(value)
    }

    /// Parses a number of `digits` decimal digits when the next character
    /// is a digit.
    fn optional_number(&mut self, digits: usize) -> Option<u32> {
        self.0
            .first()
            .is_some_and(u8::is_ascii_digit)
            .then(|| self.number(digits))
            .flatten()
    }

    /// Parses a date, `YYYYMMDD`.
    fn date(&mut self) -> Option<NaiveDate> {
        let year = self.number(4)?;
        let month = self.number(2)?;
        let day = self.number(2)?;
        NaiveDate::from_ymd_opt(year as i32, month, day)
    }

    /// Consumes the separator that starts a fraction: `.`, or `,` as well
    /// unless `canonical`.
    fn separator(&mut self, canonical: bool) -> Option<()> {
        let (&separator, rest) = self.0.split_first()?;
        let allowed = separator == b'.' || (separator == b',' && !canonical);
        allowed.then(|| self.0 = rest)
    }

    /// Parses the digits of a fraction, at least one, as billionths of the
    /// unit they qualify; digits beyond the ninth do not contribute.
    fn fraction(&mut self) -> Option<u64> {
        let digits = self
            .0
            .iter()
            .position(|character| !character.is_ascii_digit())
            .unwrap_or(self.0.len());
        if digits == 0 {
            return None;
        }
        let mut billionths = 0u64;
        for &digit in &self.0[..digits.min(9)] {
            billionths = billionths * 10 + u64::from(digit - b'0');
        }
        billionths *= 10u64.pow(9 - digits.min(9) as u32);
        self.0 = &self.0[digits..];
        Some(billionths)
    }

    /// Parses the time zone: `Z` for UTC, or a `±HHMM` offset unless
    /// `canonical`. Yields `None` when no zone is given, and fails on a
    /// malformed or disallowed one.
    fn zone(&mut self, canonical: bool) -> Option<Option<FixedOffset>> {
        let (&designator, rest) = match self.0.split_first() {
            Some(zone) => zone,
            None => return Some(None),
        };
        self.0 = rest;
        let sign = match designator {
            b'Z' => return Some(Some(FixedOffset::east_opt(0)?)),
            b'+' if !canonical => 1,
            b'-' if !canonical => -1,
            _ => return None,
        };
        let hours = self.number(2)?;
        let minutes = self.number(2)?;
        let seconds = i32::try_from(hours * 3600 + minutes * 60).ok()?;
        Some(Some(FixedOffset::east_opt(sign * seconds)?))
    }
}

/// Appends `value` as a GeneralizedTime in the canonical form: UTC, the time
/// to the second, a fraction of a second only when it is not zero and
/// without trailing zeros, and `Z`.
pub(crate) fn write_generalized_time(out: &mut Vec<u8>, value: &DateTime<FixedOffset>) {
    let value = value.naive_utc();
    write_date(out, &value.date());
    let (second, nanoseconds) = second_and_nanoseconds(value.time());
    write_time(out, value.time(), second);
    if nanoseconds > 0 {
        out.push(b'.');
        let mut fraction = nanoseconds;
        let mut digits = 9;
        while fraction % 10 == 0 {
            fraction /= 10;
            digits -= 1;
        }
        write_decimal(out, fraction, digits);
    }
    out.push(b'Z');
}

/// Appends `value` as a UTCTime in the canonical form: UTC, the year in two
/// digits, the time to the second and `Z`.
pub(crate) fn write_utc_time(out: &mut Vec<u8>, value: &DateTime<Utc>) {
    let value = value.naive_utc();
    write_pair(out, value.year().rem_euclid(100) as u32);
    write_pair(out, value.month());
    write_pair(out, value.day());
    let (second, _) = second_and_nanoseconds(value.time());
    write_time(out, value.time(), second);
    out.push(b'Z');
}

/// Appends `value` as a DATE: the year in four digits, then the month and
/// the day in two digits each.
pub(crate) fn write_date(out: &mut Vec<u8>, value: &NaiveDate) {
    let year = value.year();
    if let Ok(year) = u32::try_from(year)
        && year <= 9999
    {
        write_pair(out, year / 100);
        write_pair(out, year % 100);
    } else {
        // A year outside the four-digit range is not representable; it is
        // written with a sign and as many digits as it needs.
        out.push(if year < 0 { b'-' } else { b'+' });
        write_decimal(out, year.unsigned_abs(), 4);
    }
    write_pair(out, value.month());
    write_pair(out, value.day());
}

/// Appends the hour, minute and `second` of `value` in two digits each.
fn write_time(out: &mut Vec<u8>, value: NaiveTime, second: u32) {
    write_pair(out, value.hour());
    write_pair(out, value.minute());
    write_pair(out, second);
}

/// Appends `value`, which is below 100, in two digits.
#[inline]
fn write_pair(out: &mut Vec<u8>, value: u32) {
    debug_assert!(value < 100);
    out.extend_from_slice(&[b'0' + (value / 10) as u8, b'0' + (value % 10) as u8]);
}

/// The second and the nanoseconds within it. `chrono` represents a leap
/// second as second 59 with a billion or more nanoseconds; it is written
/// as second 60.
fn second_and_nanoseconds(value: NaiveTime) -> (u32, u32) {
    const NANOSECONDS_PER_SECOND: u32 = 1_000_000_000;
    let nanoseconds = value.nanosecond();
    if nanoseconds >= NANOSECONDS_PER_SECOND {
        (value.second() + 1, nanoseconds - NANOSECONDS_PER_SECOND)
    } else {
        (value.second(), nanoseconds)
    }
}

/// Appends `value` in decimal, zero-padded to at least `width` digits.
fn write_decimal(out: &mut Vec<u8>, mut value: u32, width: usize) {
    let digits = value.checked_ilog10().map_or(1, |log| log as usize + 1);
    let start = out.len();
    out.resize(start + width.max(digits), b'0');
    for digit in out[start..].iter_mut().rev() {
        *digit = b'0' + (value % 10) as u8;
        value /= 10;
        if value == 0 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::String;
    use chrono::TimeZone;

    fn written(write: impl FnOnce(&mut Vec<u8>)) -> String {
        let mut out = Vec::new();
        write(&mut out);
        String::from_utf8(out).unwrap()
    }

    fn generalized_time(value: DateTime<FixedOffset>) -> String {
        written(|out| write_generalized_time(out, &value))
    }

    fn utc_time(value: DateTime<Utc>) -> String {
        written(|out| write_utc_time(out, &value))
    }

    fn date(year: i32, month: u32, day: u32) -> String {
        written(|out| write_date(out, &NaiveDate::from_ymd_opt(year, month, day).unwrap()))
    }

    /// Midnight on 2020-01-01 with the given nanoseconds, which may denote a
    /// leap second.
    fn with_nanoseconds(nanoseconds: u32) -> DateTime<FixedOffset> {
        Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 59)
            .unwrap()
            .with_nanosecond(nanoseconds)
            .unwrap()
            .fixed_offset()
    }

    #[test]
    fn generalized_time_is_written_in_utc() {
        let value = FixedOffset::east_opt(5 * 3600)
            .unwrap()
            .with_ymd_and_hms(2018, 6, 13, 3, 1, 58)
            .unwrap();
        assert_eq!(generalized_time(value), "20180612220158Z");
    }

    #[test]
    fn generalized_time_omits_a_zero_fraction() {
        assert_eq!(generalized_time(with_nanoseconds(0)), "20200101000059Z");
    }

    #[test]
    fn generalized_time_fraction_drops_trailing_zeros() {
        assert_eq!(
            generalized_time(with_nanoseconds(342_000_000)),
            "20200101000059.342Z"
        );
    }

    #[test]
    fn generalized_time_fraction_keeps_leading_zeros() {
        assert_eq!(
            generalized_time(with_nanoseconds(500_000)),
            "20200101000059.0005Z"
        );
    }

    #[test]
    fn generalized_time_fraction_can_need_all_nine_digits() {
        assert_eq!(
            generalized_time(with_nanoseconds(1)),
            "20200101000059.000000001Z"
        );
    }

    #[test]
    fn generalized_time_leap_second() {
        assert_eq!(
            generalized_time(with_nanoseconds(1_000_000_000)),
            "20200101000060Z"
        );
        assert_eq!(
            generalized_time(with_nanoseconds(1_500_000_000)),
            "20200101000060.5Z"
        );
    }

    #[test]
    fn utc_time_takes_the_last_two_digits_of_the_year() {
        assert_eq!(
            utc_time(Utc.with_ymd_and_hms(2018, 1, 22, 13, 29, 0).unwrap()),
            "180122132900Z"
        );
        assert_eq!(
            utc_time(Utc.with_ymd_and_hms(2000, 2, 29, 0, 0, 0).unwrap()),
            "000229000000Z"
        );
    }

    #[test]
    fn utc_time_year_before_the_common_era() {
        assert_eq!(
            utc_time(Utc.with_ymd_and_hms(-5, 3, 4, 5, 6, 7).unwrap()),
            "950304050607Z"
        );
    }

    #[test]
    fn utc_time_leap_second_has_no_fraction() {
        assert_eq!(
            utc_time(with_nanoseconds(1_500_000_000).to_utc()),
            "200101000060Z"
        );
    }

    #[test]
    fn date_fields_are_zero_padded() {
        assert_eq!(date(1, 2, 3), "00010203");
    }

    #[test]
    fn date_year_at_the_ends_of_the_four_digit_range() {
        assert_eq!(date(0, 12, 31), "00001231");
        assert_eq!(date(9999, 12, 31), "99991231");
    }

    #[test]
    fn date_year_outside_the_four_digit_range_is_signed() {
        assert_eq!(date(10000, 1, 1), "+100000101");
        assert_eq!(date(-1, 1, 1), "-00010101");
    }

    fn utc(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> DateTime<Utc> {
        Utc.with_ymd_and_hms(year, month, day, hour, minute, second)
            .unwrap()
    }

    #[test]
    fn parses_generalized_time_to_the_hour_minute_or_second() {
        let expected = utc(2018, 6, 13, 11, 0, 0).fixed_offset();
        assert_eq!(parse_generalized_time(b"2018061311"), Some(expected));
        assert_eq!(parse_generalized_time(b"201806131100"), Some(expected));
        assert_eq!(parse_generalized_time(b"20180613110000Z"), Some(expected));
    }

    #[test]
    fn parses_generalized_time_fraction_of_the_smallest_unit() {
        let half_past_twelve = utc(2018, 6, 13, 12, 30, 0).fixed_offset();
        assert_eq!(
            parse_generalized_time(b"2018061312.5"),
            Some(half_past_twelve)
        );
        assert_eq!(
            parse_generalized_time(b"201806131229,5"),
            Some(half_past_twelve - chrono::Duration::seconds(30))
        );
        assert_eq!(
            parse_generalized_time(b"20180613122959.1234567891Z"),
            Some(
                utc(2018, 6, 13, 12, 29, 59)
                    .with_nanosecond(123_456_789)
                    .unwrap()
                    .fixed_offset()
            )
        );
    }

    #[test]
    fn parses_generalized_time_offset() {
        let offset = FixedOffset::west_opt(5 * 3600).unwrap();
        let parsed = parse_generalized_time(b"20230122130000-0500").unwrap();
        assert_eq!(parsed.offset(), &offset);
        assert_eq!(parsed, utc(2023, 1, 22, 18, 0, 0));
    }

    #[test]
    fn parses_generalized_time_leap_second() {
        let leap_second = NaiveDate::from_ymd_opt(2016, 12, 31)
            .unwrap()
            .and_hms_nano_opt(23, 59, 59, 1_500_000_000)
            .unwrap()
            .and_utc()
            .fixed_offset();
        assert_eq!(
            parse_generalized_time(b"20161231235960.5Z"),
            Some(leap_second)
        );
    }

    #[test]
    fn rejects_malformed_generalized_time() {
        for malformed in [
            &b"20180613"[..],     // no hour
            b"2018061324",        // hour out of range
            b"201806131160",      // minute out of range
            b"20180613115961",    // second out of range
            b"20180613115959.",   // fraction without digits
            b"20180613115959+05", // incomplete offset
            b"20180613115959Z1",  // trailing characters
            b"2018O613115959Z",   // not a digit
        ] {
            assert_eq!(parse_generalized_time(malformed), None, "{malformed:?}");
        }
    }

    #[test]
    fn canonical_generalized_time_requires_seconds_a_dot_and_utc() {
        assert_eq!(
            parse_canonical_generalized_time(b"20180613110000.5Z"),
            Some(
                utc(2018, 6, 13, 11, 0, 0)
                    .with_nanosecond(500_000_000)
                    .unwrap()
                    .fixed_offset()
            )
        );
        for lenient_only in [
            &b"201806131100Z"[..],
            b"20180613110000,5Z",
            b"20180613110000",
            b"20180613110000+0000",
        ] {
            assert!(
                parse_generalized_time(lenient_only).is_some(),
                "{lenient_only:?}"
            );
            assert_eq!(
                parse_canonical_generalized_time(lenient_only),
                None,
                "{lenient_only:?}"
            );
        }
    }

    #[test]
    fn parses_utc_time_to_the_minute_or_second_in_utc() {
        let expected = utc(2018, 1, 22, 13, 29, 0);
        assert_eq!(parse_utc_time(b"1801221329Z"), Some(expected));
        assert_eq!(parse_utc_time(b"180122132900Z"), Some(expected));
        assert_eq!(parse_utc_time(b"180122082900-0500"), Some(expected));
    }

    #[test]
    fn utc_time_years_from_seventy_are_in_the_twentieth_century() {
        assert_eq!(
            parse_utc_time(b"700101000000Z"),
            Some(utc(1970, 1, 1, 0, 0, 0))
        );
        assert_eq!(
            parse_utc_time(b"690101000000Z"),
            Some(utc(2069, 1, 1, 0, 0, 0))
        );
    }

    #[test]
    fn utc_time_requires_a_zone() {
        assert_eq!(parse_utc_time(b"180122132900"), None);
    }

    #[test]
    fn canonical_utc_time_requires_seconds_and_utc() {
        assert_eq!(
            parse_canonical_utc_time(b"180122132900Z"),
            Some(utc(2018, 1, 22, 13, 29, 0))
        );
        assert_eq!(parse_canonical_utc_time(b"1801221329Z"), None);
        assert_eq!(parse_canonical_utc_time(b"180122082900-0500"), None);
    }

    #[test]
    fn parses_date() {
        assert_eq!(
            parse_date(b"20121221"),
            NaiveDate::from_ymd_opt(2012, 12, 21)
        );
        assert_eq!(parse_date(b"20120230"), None);
        assert_eq!(parse_date(b"2012122"), None);
        assert_eq!(parse_date(b"20121221Z"), None);
    }
}
