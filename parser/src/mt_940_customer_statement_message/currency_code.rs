use std::error::Error;
use std::fmt::{Display, Formatter};

const CURRENCY_CODE_LENGTH: usize = 3;

#[derive(Debug, PartialEq)]
pub(super) struct CurrencyCode(String);

impl TryFrom<&str> for CurrencyCode {
    type Error = CurrencyCodeParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.len() != CURRENCY_CODE_LENGTH {
            return Err(CurrencyCodeParseError::InvalidLength);
        }
        if !value
            .chars()
            .all(|c| c.is_ascii_alphabetic() && c.is_ascii_uppercase())
        {
            return Err(CurrencyCodeParseError::InvalidFormat);
        }
        Ok(Self(value.to_string()))
    }
}

impl Display for CurrencyCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum CurrencyCodeParseError {
    InvalidLength,
    InvalidFormat,
}

impl Display for CurrencyCodeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrencyCodeParseError::InvalidLength => write!(
                f,
                "Currency code must be {} characters long",
                CURRENCY_CODE_LENGTH
            ),
            CurrencyCodeParseError::InvalidFormat => {
                write!(f, "Currency code must be uppercase alphabetic characters")
            }
        }
    }
}

impl Error for CurrencyCodeParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_currency_code_length() {
        let result = CurrencyCode::try_from("ABCD");
        assert_eq!(result, Err(CurrencyCodeParseError::InvalidLength));
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Currency code must be {} characters long",
                CURRENCY_CODE_LENGTH
            )
        );

        let result = CurrencyCode::try_from("AB");
        assert_eq!(result, Err(CurrencyCodeParseError::InvalidLength));
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Currency code must be {} characters long",
                CURRENCY_CODE_LENGTH
            )
        );
    }

    #[test]
    fn test_invalid_currency_code_format() {
        let result = CurrencyCode::try_from("123");
        assert_eq!(result, Err(CurrencyCodeParseError::InvalidFormat));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Currency code must be uppercase alphabetic characters"
        );

        let result = CurrencyCode::try_from("usd");
        assert_eq!(result, Err(CurrencyCodeParseError::InvalidFormat));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Currency code must be uppercase alphabetic characters"
        );
    }

    #[test]
    fn test_valid_currency_code() {
        let result = CurrencyCode::try_from("EUR");
        assert_eq!(result, Ok(CurrencyCode("EUR".to_string())));
        assert_eq!(result.unwrap().to_string(), "EUR");
    }
}
