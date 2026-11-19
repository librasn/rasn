//! Conversions to and from the types of the `time` crate.

use super::{GeneralizedTime, TimeConversionError, UtcTime};
use crate::types::Date;

impl From<::time::Date> for Date {
    fn from(date: ::time::Date) -> Self {
        Self::new(date.year(), date.month().into(), date.day())
            .expect("a `time::Date` is a valid date")
    }
}

impl TryFrom<Date> for ::time::Date {
    type Error = TimeConversionError;

    /// Fails when the year is out of range.
    fn try_from(date: Date) -> Result<Self, Self::Error> {
        let month = ::time::Month::try_from(date.month()).map_err(|_| TimeConversionError)?;
        Self::from_calendar_date(date.year(), month, date.day()).map_err(|_| TimeConversionError)
    }
}

impl TryFrom<::time::UtcDateTime> for UtcTime {
    type Error = TimeConversionError;

    /// Fails when the time has a fraction of a second.
    fn try_from(value: ::time::UtcDateTime) -> Result<Self, Self::Error> {
        if value.nanosecond() != 0 {
            return Err(TimeConversionError);
        }
        Self::new(
            value.date().into(),
            value.hour(),
            value.minute(),
            value.second(),
        )
        .ok_or(TimeConversionError)
    }
}

impl TryFrom<UtcTime> for ::time::UtcDateTime {
    type Error = TimeConversionError;

    /// Fails on a leap second or when the year is out of range.
    fn try_from(value: UtcTime) -> Result<Self, Self::Error> {
        let time = ::time::Time::from_hms(value.hour(), value.minute(), value.second())
            .map_err(|_| TimeConversionError)?;
        Ok(Self::new(value.date().try_into()?, time))
    }
}

impl TryFrom<::time::OffsetDateTime> for GeneralizedTime {
    type Error = TimeConversionError;

    /// Fails when the offset is not a whole number of minutes or is a day or
    /// more.
    fn try_from(value: ::time::OffsetDateTime) -> Result<Self, Self::Error> {
        let offset = value.offset().whole_seconds();
        if offset % 60 != 0 {
            return Err(TimeConversionError);
        }
        Self::new(
            value.date().into(),
            value.hour(),
            value.minute(),
            value.second(),
            value.nanosecond(),
            (offset / 60) as i16,
        )
        .ok_or(TimeConversionError)
    }
}

impl TryFrom<GeneralizedTime> for ::time::OffsetDateTime {
    type Error = TimeConversionError;

    /// Fails on a leap second or when the year is out of range.
    fn try_from(value: GeneralizedTime) -> Result<Self, Self::Error> {
        let time = ::time::Time::from_hms_nano(
            value.hour(),
            value.minute(),
            value.second(),
            value.nanosecond(),
        )
        .map_err(|_| TimeConversionError)?;
        let offset = ::time::UtcOffset::from_whole_seconds(i32::from(value.offset()) * 60)
            .map_err(|_| TimeConversionError)?;
        Ok(Self::new_in_offset(value.date().try_into()?, time, offset))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(year: i32, month: u8, day: u8) -> Date {
        Date::new(year, month, day).unwrap()
    }

    #[test]
    fn date_round_trips() {
        let value = date(2024, 2, 29);
        let converted = ::time::Date::try_from(value).unwrap();
        assert_eq!(
            converted,
            ::time::Date::from_calendar_date(2024, ::time::Month::February, 29).unwrap()
        );
        assert_eq!(Date::from(converted), value);
        assert_eq!(
            ::time::Date::try_from(date(10_000, 1, 1)),
            Err(TimeConversionError)
        );
    }

    #[test]
    fn utc_time_round_trips() {
        let value = UtcTime::new(date(2018, 1, 22), 13, 29, 0).unwrap();
        let converted = ::time::UtcDateTime::try_from(value).unwrap();
        assert_eq!(converted.unix_timestamp(), 1_516_627_740);
        assert_eq!(UtcTime::try_from(converted), Ok(value));
    }

    #[test]
    fn utc_time_rejects_what_it_cannot_hold() {
        let leap = UtcTime::new(date(2016, 12, 31), 23, 59, 60).unwrap();
        assert_eq!(
            ::time::UtcDateTime::try_from(leap),
            Err(TimeConversionError)
        );
        let fraction = ::time::UtcDateTime::from_unix_timestamp_nanos(1).unwrap();
        assert_eq!(UtcTime::try_from(fraction), Err(TimeConversionError));
    }

    #[test]
    fn generalized_time_round_trips_with_its_offset() {
        let value = GeneralizedTime::new(date(2023, 1, 22), 13, 0, 0, 5, -300).unwrap();
        let converted = ::time::OffsetDateTime::try_from(value).unwrap();
        assert_eq!(converted.unix_timestamp_nanos(), 1_674_410_400_000_000_005);
        assert_eq!(converted.offset().whole_minutes(), -300);
        let back = GeneralizedTime::try_from(converted).unwrap();
        assert_eq!((back.hour(), back.offset()), (13, -300));
        assert_eq!(back, value);
    }

    #[test]
    fn generalized_time_rejects_what_it_cannot_hold() {
        let leap = GeneralizedTime::new(date(2016, 12, 31), 23, 59, 60, 0, 0).unwrap();
        assert_eq!(
            ::time::OffsetDateTime::try_from(leap),
            Err(TimeConversionError)
        );
        let odd_offset = ::time::OffsetDateTime::UNIX_EPOCH
            .to_offset(::time::UtcOffset::from_whole_seconds(30).unwrap());
        assert_eq!(
            GeneralizedTime::try_from(odd_offset),
            Err(TimeConversionError)
        );
    }
}
