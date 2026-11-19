use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
};

use super::Date;

#[cfg(feature = "jiff")]
mod jiff;
#[cfg(feature = "time")]
mod time;

const MINUTES_PER_DAY: i64 = 24 * 60;
const NANOSECONDS_PER_SECOND: u32 = 1_000_000_000;

/// The `UTCTime` type: a time in UTC, to the second.
///
/// ASN.1 gives the year in two digits. Decoding reads 70 to 99 as 1970 to
/// 1999 and 00 to 69 as 2000 to 2069; encoding writes the last two digits.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UtcTime {
    date: Date,
    hour: u8,
    minute: u8,
    second: u8,
}

impl UtcTime {
    /// `hour:minute:second` UTC on `date`, unless a field is out of range.
    /// A `second` of 60 is a leap second.
    #[must_use]
    pub const fn new(date: Date, hour: u8, minute: u8, second: u8) -> Option<Self> {
        if hour >= 24 || minute >= 60 || second > 60 {
            return None;
        }
        Some(Self {
            date,
            hour,
            minute,
            second,
        })
    }

    /// The date.
    #[must_use]
    pub const fn date(self) -> Date {
        self.date
    }

    /// The hour, from 0 to 23.
    #[must_use]
    pub const fn hour(self) -> u8 {
        self.hour
    }

    /// The minute, from 0 to 59.
    #[must_use]
    pub const fn minute(self) -> u8 {
        self.minute
    }

    /// The second, from 0 to 60 for a leap second.
    #[must_use]
    pub const fn second(self) -> u8 {
        self.second
    }
}

impl fmt::Display for UtcTime {
    /// Formats the time as RFC 3339 does, `YYYY-MM-DDTHH:MM:SSZ`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}T{:02}:{:02}:{:02}Z",
            self.date, self.hour, self.minute, self.second
        )
    }
}

impl From<UtcTime> for GeneralizedTime {
    fn from(value: UtcTime) -> Self {
        Self {
            date: value.date,
            hour: value.hour,
            minute: value.minute,
            second: value.second,
            nanosecond: 0,
            offset: 0,
        }
    }
}

/// The `GeneralizedTime` type: a date and time, to the nanosecond, at an
/// offset from UTC.
///
/// Values are equal, ordered and hashed by the instant they denote, so the
/// same instant at different offsets compares equal.
#[derive(Clone, Copy, Debug)]
pub struct GeneralizedTime {
    date: Date,
    hour: u8,
    minute: u8,
    second: u8,
    nanosecond: u32,
    offset: i16,
}

impl GeneralizedTime {
    /// `hour:minute:second` and `nanosecond` on `date`, at `offset` minutes
    /// east of UTC, unless a field is out of range. A `second` of 60 is a
    /// leap second; the offset is less than a day either way.
    #[must_use]
    pub const fn new(
        date: Date,
        hour: u8,
        minute: u8,
        second: u8,
        nanosecond: u32,
        offset: i16,
    ) -> Option<Self> {
        if hour >= 24
            || minute >= 60
            || second > 60
            || nanosecond >= NANOSECONDS_PER_SECOND
            || offset.unsigned_abs() as i64 >= MINUTES_PER_DAY
        {
            return None;
        }
        Some(Self {
            date,
            hour,
            minute,
            second,
            nanosecond,
            offset,
        })
    }

    /// The date at the offset.
    #[must_use]
    pub const fn date(self) -> Date {
        self.date
    }

    /// The hour at the offset, from 0 to 23.
    #[must_use]
    pub const fn hour(self) -> u8 {
        self.hour
    }

    /// The minute at the offset, from 0 to 59.
    #[must_use]
    pub const fn minute(self) -> u8 {
        self.minute
    }

    /// The second, from 0 to 60 for a leap second.
    #[must_use]
    pub const fn second(self) -> u8 {
        self.second
    }

    /// The nanoseconds into the second.
    #[must_use]
    pub const fn nanosecond(self) -> u32 {
        self.nanosecond
    }

    /// The offset from UTC in minutes, east positive.
    #[must_use]
    pub const fn offset(self) -> i16 {
        self.offset
    }

    /// The minutes from 1970-01-01 00:00 UTC to the start of this minute.
    pub(crate) const fn utc_minutes(self) -> i64 {
        self.date.days_since_epoch() * MINUTES_PER_DAY
            + (self.hour as i64 * 60 + self.minute as i64)
            - self.offset as i64
    }

    fn instant(self) -> (i64, u8, u32) {
        (self.utc_minutes(), self.second, self.nanosecond)
    }
}

impl PartialEq for GeneralizedTime {
    fn eq(&self, other: &Self) -> bool {
        self.instant() == other.instant()
    }
}

impl Eq for GeneralizedTime {}

impl PartialOrd for GeneralizedTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for GeneralizedTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.instant().cmp(&other.instant())
    }
}

impl Hash for GeneralizedTime {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.instant().hash(state);
    }
}

impl fmt::Display for GeneralizedTime {
    /// Formats the time as RFC 3339 does, such as
    /// `YYYY-MM-DDTHH:MM:SS.sss+HH:MM`, with `Z` for UTC.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}T{:02}:{:02}:{:02}",
            self.date, self.hour, self.minute, self.second
        )?;
        if self.nanosecond > 0 {
            let mut fraction = self.nanosecond;
            let mut digits = 9;
            while fraction.is_multiple_of(10) {
                fraction /= 10;
                digits -= 1;
            }
            write!(f, ".{fraction:0digits$}")?;
        }
        match self.offset {
            0 => f.write_str("Z"),
            offset => {
                let sign = if offset < 0 { '-' } else { '+' };
                let minutes = offset.unsigned_abs();
                write!(f, "{sign}{:02}:{:02}", minutes / 60, minutes % 60)
            }
        }
    }
}

/// The error when a time does not fit the type it is converted to, such as
/// a leap second, a year out of range, a fraction of a second in a
/// [`UtcTime`] or an offset that is not a whole number of minutes.
#[cfg(any(feature = "time", feature = "jiff"))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct TimeConversionError;

#[cfg(any(feature = "time", feature = "jiff"))]
impl fmt::Display for TimeConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("the time does not fit the target type")
    }
}

#[cfg(any(feature = "time", feature = "jiff"))]
impl core::error::Error for TimeConversionError {}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;
    use core::hash::BuildHasher;

    fn date(year: i32, month: u8, day: u8) -> Date {
        Date::new(year, month, day).unwrap()
    }

    fn generalized(
        (year, month, day): (i32, u8, u8),
        (hour, minute, second): (u8, u8, u8),
        nanosecond: u32,
        offset: i16,
    ) -> GeneralizedTime {
        GeneralizedTime::new(
            date(year, month, day),
            hour,
            minute,
            second,
            nanosecond,
            offset,
        )
        .unwrap()
    }

    #[test]
    fn rejects_fields_out_of_range() {
        let day = date(2024, 1, 1);
        assert!(UtcTime::new(day, 23, 59, 60).is_some());
        assert!(UtcTime::new(day, 24, 0, 0).is_none());
        assert!(UtcTime::new(day, 0, 60, 0).is_none());
        assert!(UtcTime::new(day, 0, 0, 61).is_none());
        assert!(GeneralizedTime::new(day, 0, 0, 0, 999_999_999, 1439).is_some());
        assert!(GeneralizedTime::new(day, 0, 0, 0, 999_999_999, -1439).is_some());
        assert!(GeneralizedTime::new(day, 0, 0, 0, 1_000_000_000, 0).is_none());
        assert!(GeneralizedTime::new(day, 0, 0, 0, 0, 1440).is_none());
        assert!(GeneralizedTime::new(day, 0, 0, 0, 0, -1440).is_none());
    }

    #[test]
    fn generalized_time_compares_instants() {
        let utc = generalized((2023, 1, 22), (18, 0, 0), 0, 0);
        let eastern = generalized((2023, 1, 22), (13, 0, 0), 0, -300);
        let tokyo = generalized((2023, 1, 23), (3, 0, 0), 0, 540);
        assert_eq!(utc, eastern);
        assert_eq!(utc, tokyo);
        let hasher = std::hash::RandomState::new();
        assert_eq!(hasher.hash_one(utc), hasher.hash_one(tokyo));
        assert!(eastern < generalized((2023, 1, 22), (18, 0, 0), 1, 0));
    }

    #[test]
    fn utc_time_converts_to_generalized_time() {
        let utc = UtcTime::new(date(2016, 12, 31), 23, 59, 60).unwrap();
        let generalized_utc = GeneralizedTime::from(utc);
        assert_eq!(
            generalized_utc,
            generalized((2016, 12, 31), (23, 59, 60), 0, 0)
        );
        assert_eq!(generalized_utc.offset(), 0);
    }

    #[test]
    fn leap_second_sorts_between_its_neighbours() {
        let leap = generalized((2016, 12, 31), (23, 59, 60), 0, 0);
        assert!(generalized((2016, 12, 31), (23, 59, 59), 999_999_999, 0) < leap);
        assert!(leap < generalized((2016, 12, 31), (23, 59, 60), 500_000_000, 0));
        assert!(leap < generalized((2017, 1, 1), (0, 0, 0), 0, 0));
        assert_ne!(leap, generalized((2017, 1, 1), (0, 0, 0), 0, 0));
    }

    #[test]
    fn displays_as_rfc_3339() {
        assert_eq!(
            UtcTime::new(date(2018, 1, 22), 13, 29, 0)
                .unwrap()
                .to_string(),
            "2018-01-22T13:29:00Z"
        );
        assert_eq!(
            generalized((2018, 6, 13), (3, 1, 58), 0, 300).to_string(),
            "2018-06-13T03:01:58+05:00"
        );
        assert_eq!(
            generalized((2018, 6, 13), (3, 1, 58), 500_000, -90).to_string(),
            "2018-06-13T03:01:58.0005-01:30"
        );
        assert_eq!(
            generalized((2016, 12, 31), (23, 59, 60), 1, 0).to_string(),
            "2016-12-31T23:59:60.000000001Z"
        );
    }
}
