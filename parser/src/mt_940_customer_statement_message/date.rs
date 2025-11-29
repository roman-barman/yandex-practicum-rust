use chrono::{Datelike, NaiveDate};
use std::error::Error;
use std::fmt::Display;

pub(super) const DATE_LENGTH: usize = 6;

#[derive(Debug, PartialEq)]
pub(super) struct Date(NaiveDate);

impl Date {
    pub(super) fn new(date: NaiveDate) -> Self {
        Self(date)
    }

    pub(super) fn ymd_date(&self) -> (i32, u32, u32) {
        (self.0.year(), self.0.month(), self.0.day())
    }

    pub(super) fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.0.format("%y%m%d").to_string().as_bytes())
    }

    pub(super) fn write_without_year_to<W: std::io::Write>(
        &self,
        writer: &mut W,
    ) -> std::io::Result<()> {
        writer.write_all(self.0.format("%m%d").to_string().as_bytes())
    }
}

impl TryFrom<&str> for Date {
    type Error = DateParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(DateParseError::Empty);
        }
        if value.len() != DATE_LENGTH {
            return Err(DateParseError::InvalidFormat);
        }

        let year = value
            .chars()
            .take(2)
            .collect::<String>()
            .parse::<i32>()
            .map_err(|_| DateParseError::InvalidFormat)?;
        let month = value
            .chars()
            .skip(2)
            .take(2)
            .collect::<String>()
            .parse::<u32>()
            .map_err(|_| DateParseError::InvalidFormat)?;
        let day = value
            .chars()
            .skip(4)
            .take(2)
            .collect::<String>()
            .parse::<u32>()
            .map_err(|_| DateParseError::InvalidFormat)?;
        let date =
            NaiveDate::from_ymd_opt(2000 + year, month, day).ok_or(DateParseError::InvalidValue)?;

        Ok(Self(date))
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d"))
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum DateParseError {
    Empty,
    InvalidFormat,
    InvalidValue,
}

impl Display for DateParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateParseError::Empty => write!(f, "Date cannot be empty"),
            DateParseError::InvalidFormat => write!(f, "Invalid date format"),
            DateParseError::InvalidValue => write!(f, "Invalid date"),
        }
    }
}

impl Error for DateParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_date_write_to() {
        let mut buffer = Cursor::new(Vec::new());
        Date(NaiveDate::from_ymd_opt(2021, 1, 1).unwrap())
            .write_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer.get_ref(), b"210101");
    }

    #[test]
    fn test_date_write_without_year_to() {
        let mut buffer = Cursor::new(Vec::new());
        Date(NaiveDate::from_ymd_opt(2021, 1, 1).unwrap())
            .write_without_year_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer.get_ref(), b"0101");
    }

    #[test]
    fn test_empty_date() {
        let result = Date::try_from("");
        assert_eq!(result, Err(DateParseError::Empty));
        assert_eq!(result.unwrap_err().to_string(), "Date cannot be empty");
    }

    #[test]
    fn test_invalid_date_format() {
        let result = Date::try_from("20211301");
        assert_eq!(result, Err(DateParseError::InvalidFormat));
        assert_eq!(result.unwrap_err().to_string(), "Invalid date format");

        let result = Date::try_from("2o1130");
        assert_eq!(result, Err(DateParseError::InvalidFormat));
        assert_eq!(result.unwrap_err().to_string(), "Invalid date format");

        let result = Date::try_from("21o130");
        assert_eq!(result, Err(DateParseError::InvalidFormat));
        assert_eq!(result.unwrap_err().to_string(), "Invalid date format");

        let result = Date::try_from("21113o");
        assert_eq!(result, Err(DateParseError::InvalidFormat));
        assert_eq!(result.unwrap_err().to_string(), "Invalid date format");
    }

    #[test]
    fn test_invalid_date() {
        let result = Date::try_from("210231");
        assert_eq!(result, Err(DateParseError::InvalidValue));
        assert_eq!(result.unwrap_err().to_string(), "Invalid date");
    }

    #[test]
    fn test_valid_date() {
        let result = Date::try_from("000101");
        assert_eq!(
            result,
            Ok(Date(NaiveDate::from_ymd_opt(2000, 1, 1).unwrap()))
        );
        assert_eq!(result.unwrap().to_string(), "2000-01-01");
    }
}
