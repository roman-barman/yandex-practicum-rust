use rust_decimal::Decimal;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const AMOUNT_MAX_LENGTH: usize = 15;

#[derive(Debug, PartialEq)]
pub(super) struct Amount(Decimal);

impl TryFrom<&str> for Amount {
    type Error = AmountParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim().replace(',', ".");
        if value.is_empty() {
            return Err(AmountParseError::Empty);
        }

        if value.len() > AMOUNT_MAX_LENGTH {
            return Err(AmountParseError::TooLong);
        }

        if !value.contains('.') {
            return Err(AmountParseError::InvalidFormat);
        }

        let decimal = Decimal::from_str(&value).map_err(|_| AmountParseError::InvalidFormat)?;
        Ok(Self(decimal))
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum AmountParseError {
    Empty,
    TooLong,
    InvalidFormat,
}

impl Display for AmountParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AmountParseError::Empty => write!(f, "Amount cannot be empty"),
            AmountParseError::TooLong => write!(
                f,
                "Amount exceeds maximum length of {} characters",
                AMOUNT_MAX_LENGTH
            ),
            AmountParseError::InvalidFormat => write!(f, "Invalid amount format"),
        }
    }
}

impl Error for AmountParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_amount() {
        let result = Amount::try_from("");
        assert_eq!(result, Err(AmountParseError::Empty));
        assert_eq!(result.unwrap_err().to_string(), "Amount cannot be empty");
    }

    #[test]
    fn test_amount_too_long() {
        let result = Amount::try_from("123456789,123456789");
        assert_eq!(result, Err(AmountParseError::TooLong));
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Amount exceeds maximum length of {} characters",
                AMOUNT_MAX_LENGTH
            )
        );
    }

    #[test]
    fn test_invalid_amount_format() {
        let result = Amount::try_from("12345678");
        assert_eq!(result, Err(AmountParseError::InvalidFormat));
        assert_eq!(result.unwrap_err().to_string(), "Invalid amount format");

        let result = Amount::try_from("1234567a,0");
        assert_eq!(result, Err(AmountParseError::InvalidFormat));
        assert_eq!(result.unwrap_err().to_string(), "Invalid amount format");
    }

    #[test]
    fn test_valid_amount() {
        let result = Amount::try_from("12345678.09");
        assert_eq!(result, Ok(Amount(Decimal::new(1234567809, 2))));
        assert_eq!(result.unwrap().to_string(), "12345678.09");
    }
}
