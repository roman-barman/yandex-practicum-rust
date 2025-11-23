use std::error::Error;
use std::fmt::{Display, Formatter};

pub(super) const IDENTIFICATION_CODE_LENGTH: usize = 3;

#[derive(Debug, PartialEq)]
pub(super) struct IdentificationCode(String);

impl TryFrom<&str> for IdentificationCode {
    type Error = IdentificationCodeParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(IdentificationCodeParseError::Empty);
        }
        if value.len() != IDENTIFICATION_CODE_LENGTH
            || (value
                .chars()
                .any(|c| c.is_ascii_digit() && value.chars().any(|c| c.is_ascii_alphabetic())))
        {
            return Err(IdentificationCodeParseError::InvalidFormat);
        }
        Ok(Self(value.to_string()))
    }
}

impl Display for IdentificationCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum IdentificationCodeParseError {
    Empty,
    InvalidFormat,
}

impl Display for IdentificationCodeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IdentificationCodeParseError::Empty => write!(f, "Identification code cannot be empty"),
            IdentificationCodeParseError::InvalidFormat => {
                write!(f, "Invalid identification code format")
            }
        }
    }
}

impl Error for IdentificationCodeParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_identification_code() {
        let result = IdentificationCode::try_from("");
        assert_eq!(result, Err(IdentificationCodeParseError::Empty));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Identification code cannot be empty"
        );
    }

    #[test]
    fn test_invalid_format_identification_code() {
        let result = IdentificationCode::try_from("123A");
        assert_eq!(result, Err(IdentificationCodeParseError::InvalidFormat));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid identification code format"
        );

        let result = IdentificationCode::try_from("12A");
        assert_eq!(result, Err(IdentificationCodeParseError::InvalidFormat));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid identification code format"
        );
    }

    #[test]
    fn test_valid_identification_code() {
        let result = IdentificationCode::try_from("123");
        assert_eq!(result, Ok(IdentificationCode("123".to_string())));
        assert_eq!(result.unwrap().to_string(), "123");

        let result = IdentificationCode::try_from("ABC");
        assert_eq!(result, Ok(IdentificationCode("ABC".to_string())));
        assert_eq!(result.unwrap().to_string(), "ABC");
    }
}
