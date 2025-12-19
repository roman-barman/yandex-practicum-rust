use rust_decimal::Decimal;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

const AMOUNT_MAX_LENGTH: usize = 15;

#[derive(Debug, PartialEq)]
pub(crate) struct Amount(Decimal);

impl AsRef<Decimal> for Amount {
    fn as_ref(&self) -> &Decimal {
        &self.0
    }
}

impl Amount {
    pub(super) fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        let mut data = self.0.to_string().replace('.', ",");
        if !data.contains(',') {
            data.push_str(",0")
        }
        writer.write_all(data.as_bytes())
    }

    pub(crate) fn new(value: Decimal) -> Self {
        Self(value)
    }
}

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

#[derive(Debug, PartialEq, Error)]
pub(crate) enum AmountParseError {
    #[error("Amount cannot be empty")]
    Empty,
    #[error("Amount cannot be longer than {} characters", AMOUNT_MAX_LENGTH)]
    TooLong,
    #[error("Invalid amount format")]
    InvalidFormat,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_amount_write_to() {
        let mut buffer = Cursor::new(Vec::new());
        Amount(Decimal::new(1234567809, 2))
            .write_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer.get_ref(), b"12345678,09");

        let mut buffer = Cursor::new(Vec::new());
        Amount(Decimal::new(1234567809, 0))
            .write_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer.get_ref(), b"1234567809,0");
    }

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
                "Amount cannot be longer than {} characters",
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
