//! Conversions to and from the types of the `jiff` crate.

use ::jiff::{Timestamp, civil, tz::Offset};

use super::{GeneralizedTime, TimeConversionError, UtcTime};
use crate::types::Date;

impl From<civil::Date> for Date {
    fn from(date: civil::Date) -> Self {
        Self::new(date.year().into(), date.month() as u8, date.day() as u8)
            .expect("a `jiff::civil::Date` is a valid date")
    }
}

impl TryFrom<Date> for civil::Date {
    type Error = TimeConversionError;

    /// Fails when the year is out of range.
    fn try_from(date: Date) -> Result<Self, Self::Error> {
        let year = i16::try_from(date.year()).map_err(|_| TimeConversionError)?;
        Self::new(year, date.month() as i8, date.day() as i8).map_err(|_| TimeConversionError)
    }
}

/// `hour:minute:second` and `nanosecond` on `date`, unless that is a leap
/// second or out of range.
fn civil_date_time(
    date: Date,
    hour: u8,
    minute: u8,
    second: u8,
    nanosecond: u32,
) -> Result<civil::DateTime, TimeConversionError> {
    let time = civil::Time::new(hour as i8, minute as i8, second as i8, nanosecond as i32)
        .map_err(|_| TimeConversionError)?;
    Ok(civil::DateTime::from_parts(date.try_into()?, time))
}

impl TryFrom<Timestamp> for UtcTime {
    type Error = TimeConversionError;

    /// Fails when the time has a fraction of a second.
    fn try_from(value: Timestamp) -> Result<Self, Self::Error> {
        let utc = Offset::UTC.to_datetime(value);
        if utc.subsec_nanosecond() != 0 {
            return Err(TimeConversionError);
        }
        Self::new(
            utc.date().into(),
            utc.hour() as u8,
            utc.minute() as u8,
            utc.second() as u8,
        )
        .ok_or(TimeConversionError)
    }
}

impl TryFrom<UtcTime> for Timestamp {
    type Error = TimeConversionError;

    /// Fails on a leap second or when the year is out of range.
    fn try_from(value: UtcTime) -> Result<Self, Self::Error> {
        let utc = civil_date_time(
            value.date(),
            value.hour(),
            value.minute(),
            value.second(),
            0,
        )?;
        Offset::UTC
            .to_timestamp(utc)
            .map_err(|_| TimeConversionError)
    }
}

impl From<Timestamp> for GeneralizedTime {
    /// The time in UTC.
    fn from(value: Timestamp) -> Self {
        let utc = Offset::UTC.to_datetime(value);
        Self::new(
            utc.date().into(),
            utc.hour() as u8,
            utc.minute() as u8,
            utc.second() as u8,
            utc.subsec_nanosecond() as u32,
            0,
        )
        .expect("a `jiff::Timestamp` is a valid time")
    }
}

impl TryFrom<GeneralizedTime> for Timestamp {
    type Error = TimeConversionError;

    /// Fails on a leap second or when the year is out of range.
    fn try_from(value: GeneralizedTime) -> Result<Self, Self::Error> {
        let local = civil_date_time(
            value.date(),
            value.hour(),
            value.minute(),
            value.second(),
            value.nanosecond(),
        )?;
        Offset::from_seconds(i32::from(value.offset()) * 60)
            .and_then(|offset| offset.to_timestamp(local))
            .map_err(|_| TimeConversionError)
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
        let converted = civil::Date::try_from(value).unwrap();
        assert_eq!(converted, civil::date(2024, 2, 29));
        assert_eq!(Date::from(converted), value);
        assert_eq!(
            civil::Date::try_from(date(10_000, 1, 1)),
            Err(TimeConversionError)
        );
    }

    #[test]
    fn utc_time_round_trips() {
        let value = UtcTime::new(date(2018, 1, 22), 13, 29, 0).unwrap();
        let converted = Timestamp::try_from(value).unwrap();
        assert_eq!(converted.as_second(), 1_516_627_740);
        assert_eq!(UtcTime::try_from(converted), Ok(value));
    }

    #[test]
    fn utc_time_rejects_what_it_cannot_hold() {
        let leap = UtcTime::new(date(2016, 12, 31), 23, 59, 60).unwrap();
        assert_eq!(Timestamp::try_from(leap), Err(TimeConversionError));
        let fraction = Timestamp::new(0, 1).unwrap();
        assert_eq!(UtcTime::try_from(fraction), Err(TimeConversionError));
    }

    #[test]
    fn generalized_time_converts_the_instant() {
        let value = GeneralizedTime::new(date(2023, 1, 22), 13, 0, 0, 5, -300).unwrap();
        let converted = Timestamp::try_from(value).unwrap();
        assert_eq!(converted, Timestamp::new(1_674_410_400, 5).unwrap());
        let back = GeneralizedTime::from(converted);
        assert_eq!((back.hour(), back.offset()), (18, 0));
        assert_eq!(back, value);
        let before_epoch = Timestamp::new(-1, 999_999_999).unwrap();
        assert_eq!(
            GeneralizedTime::from(before_epoch),
            GeneralizedTime::new(date(1969, 12, 31), 23, 59, 59, 999_999_999, 0).unwrap()
        );
    }

    #[test]
    fn generalized_time_rejects_a_leap_second() {
        let leap = GeneralizedTime::new(date(2016, 12, 31), 23, 59, 60, 0, 0).unwrap();
        assert_eq!(Timestamp::try_from(leap), Err(TimeConversionError));
    }
}
