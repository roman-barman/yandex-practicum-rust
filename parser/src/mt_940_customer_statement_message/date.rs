use chrono::NaiveDate;
use std::error::Error;
use std::fmt::Display;

const DATE_LENGTH: usize = 6;

#[derive(Debug, PartialEq)]
pub(super) struct Date(NaiveDate);

impl Date {
    pub fn new(date: NaiveDate) -> Self {
        Self(date)
    }
}

impl TryFrom<&str> for Date {
    type Error = DateParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();

        if value.len() != DATE_LENGTH {
            return Err(DateParseError::InvalidLength);
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
    InvalidLength,
    InvalidFormat,
    InvalidValue,
}

impl std::fmt::Display for DateParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateParseError::InvalidFormat => write!(f, "Invalid date format"),
            DateParseError::InvalidValue => write!(f, "Invalid date"),
            DateParseError::InvalidLength => write!(f, "Invalid date length"),
        }
    }
}

impl Error for DateParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_date_length() {
        let result = Date::try_from("20211301");
        assert_eq!(result, Err(DateParseError::InvalidLength));
        assert_eq!(result.unwrap_err().to_string(), "Invalid date length");
    }

    #[test]
    fn test_invalid_date_format() {
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
