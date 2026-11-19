use core::fmt;

use crate::prelude::Constraints;
use crate::{AsnType, Decode, Decoder, Encode, Encoder, types::Tag};

use super::Identifier;

/// The `DATE` type: a day in the proleptic Gregorian calendar.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date {
    year: i32,
    month: u8,
    day: u8,
}

impl Date {
    /// The date `year`-`month`-`day`, unless there is no such day.
    #[must_use]
    pub const fn new(year: i32, month: u8, day: u8) -> Option<Self> {
        if month == 0 || month > 12 || day == 0 || day > days_in_month(year, month) {
            return None;
        }
        Some(Self { year, month, day })
    }

    /// The year.
    #[must_use]
    pub const fn year(self) -> i32 {
        self.year
    }

    /// The month, from 1 to 12.
    #[must_use]
    pub const fn month(self) -> u8 {
        self.month
    }

    /// The day of the month, from 1.
    #[must_use]
    pub const fn day(self) -> u8 {
        self.day
    }

    /// The number of days from 1970-01-01 to this date.
    pub(crate) const fn days_since_epoch(self) -> i64 {
        days_from_civil(self.year as i64, self.month, self.day)
    }
}

impl fmt::Display for Date {
    /// Formats the date as ISO 8601 does, `YYYY-MM-DD`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write_year(f, self.year)?;
        write!(f, "-{:02}-{:02}", self.month, self.day)
    }
}

/// Writes `year` in four digits, or with a sign outside 0 to 9999.
fn write_year(f: &mut fmt::Formatter<'_>, year: i32) -> fmt::Result {
    if (0..=9999).contains(&year) {
        write!(f, "{year:04}")
    } else {
        write!(f, "{year:+05}")
    }
}

const fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

const fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        2 if is_leap_year(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

// The two conversions below are Howard Hinnant's `days_from_civil` and
// `civil_from_days`, which count in 400-year eras starting on 1 March.

/// The number of days from 1970-01-01 to `year`-`month`-`day`.
pub(crate) const fn days_from_civil(year: i64, month: u8, day: u8) -> i64 {
    let year = year - (month <= 2) as i64;
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let month = month as i64;
    let day_of_year =
        (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day as i64 - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

/// The year, month and day `days` after 1970-01-01.
pub(crate) const fn civil_from_days(days: i64) -> (i64, u8, u8) {
    let days = days + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_from_march = (5 * day_of_year + 2) / 153;
    let day = (day_of_year - (153 * month_from_march + 2) / 5 + 1) as u8;
    let month = if month_from_march < 10 {
        month_from_march + 3
    } else {
        month_from_march - 9
    } as u8;
    (year_of_era + era * 400 + (month <= 2) as i64, month, day)
}

impl AsnType for Date {
    const TAG: Tag = Tag::DATE;
    const IDENTIFIER: Identifier = Identifier::DATE;
}

impl Decode for Date {
    fn decode_with_tag_and_constraints<D: Decoder>(
        decoder: &mut D,
        tag: Tag,
        _constraints: &Constraints,
    ) -> Result<Self, D::Error> {
        decoder.decode_date(tag)
    }
}

impl Encode for Date {
    fn encode_with_tag_and_constraints<'b, E: Encoder<'b>>(
        &self,
        encoder: &mut E,
        tag: Tag,
        _constraints: &Constraints,
        identifier: Identifier,
    ) -> Result<(), E::Error> {
        encoder.encode_date(tag, self, identifier).map(drop)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn rejects_days_that_do_not_exist() {
        assert!(Date::new(2024, 2, 29).is_some());
        assert!(Date::new(2023, 2, 29).is_none());
        assert!(Date::new(1900, 2, 29).is_none());
        assert!(Date::new(2000, 2, 29).is_some());
        assert!(Date::new(-4, 2, 29).is_some());
        assert!(Date::new(2024, 4, 31).is_none());
        assert!(Date::new(2024, 0, 1).is_none());
        assert!(Date::new(2024, 13, 1).is_none());
        assert!(Date::new(2024, 1, 0).is_none());
    }

    #[test]
    fn counts_days_from_the_epoch() {
        for (year, month, day, days) in [
            (1970, 1, 1, 0),
            (1969, 12, 31, -1),
            (2000, 3, 1, 11_017),
            (2024, 2, 29, 19_782),
            (0, 1, 1, -719_528),
            (-1, 12, 31, -719_529),
        ] {
            let date = Date::new(year, month, day).unwrap();
            assert_eq!(date.days_since_epoch(), days, "{date}");
            assert_eq!(civil_from_days(days), (year.into(), month, day));
        }
        for (year, month, day) in [(i32::MIN, 1, 1), (i32::MAX, 12, 31)] {
            let days = Date::new(year, month, day).unwrap().days_since_epoch();
            assert_eq!(civil_from_days(days), (year.into(), month, day));
        }
        let last = Date::new(i32::MAX, 12, 31).unwrap().days_since_epoch();
        assert_eq!(civil_from_days(last + 1), (i64::from(i32::MAX) + 1, 1, 1));
    }

    #[test]
    fn displays_as_iso_8601() {
        assert_eq!(Date::new(2024, 4, 4).unwrap().to_string(), "2024-04-04");
        assert_eq!(Date::new(1, 2, 3).unwrap().to_string(), "0001-02-03");
        assert_eq!(Date::new(10000, 1, 1).unwrap().to_string(), "+10000-01-01");
        assert_eq!(Date::new(-1, 1, 1).unwrap().to_string(), "-0001-01-01");
    }
}
