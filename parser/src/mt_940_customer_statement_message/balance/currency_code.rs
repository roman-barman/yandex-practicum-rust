use std::fmt::{Display, Formatter};
use thiserror::Error;

const CURRENCY_CODE_LENGTH: usize = 3;

#[derive(Debug, PartialEq)]
pub(crate) struct CurrencyCode(String);

impl AsRef<str> for CurrencyCode {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl CurrencyCode {
    pub(super) fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.0.as_bytes())
    }
}

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

#[derive(Debug, PartialEq, Error)]
pub(crate) enum CurrencyCodeParseError {
    #[error("Currency code must be {} characters long", CURRENCY_CODE_LENGTH)]
    InvalidLength,
    #[error("Currency code must be uppercase alphabetic characters")]
    InvalidFormat,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_currency_code_write_to() {
        let mut buffer = Cursor::new(Vec::new());
        CurrencyCode("EUR".to_string())
            .write_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer.get_ref(), b"EUR");
    }

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
