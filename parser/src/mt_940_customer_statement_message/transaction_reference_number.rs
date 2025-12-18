use std::fmt::Display;
use std::io::Write;
use thiserror::Error;

const TRANSACTION_REFERENCE_NUMBER_MAX_LENGTH: usize = 16;

#[derive(Debug, PartialEq)]
pub(crate) struct TransactionReferenceNumber(String);

impl AsRef<str> for TransactionReferenceNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TransactionReferenceNumber {
    pub(super) fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.0.as_bytes())
    }
}

impl TryFrom<&str> for TransactionReferenceNumber {
    type Error = TransactionReferenceNumberParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(TransactionReferenceNumberParseError::Empty);
        }

        if value.len() > TRANSACTION_REFERENCE_NUMBER_MAX_LENGTH {
            return Err(TransactionReferenceNumberParseError::TooLong);
        }

        if value.starts_with("/") || value.ends_with("/") || value.contains("//") {
            return Err(TransactionReferenceNumberParseError::InvalidFormat);
        }

        Ok(Self(value.to_string()))
    }
}

impl Display for TransactionReferenceNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Error)]
pub(crate) enum TransactionReferenceNumberParseError {
    #[error("Transaction reference number cannot be empty")]
    Empty,
    #[error(
        "Transaction reference number cannot be longer than {} characters",
        TRANSACTION_REFERENCE_NUMBER_MAX_LENGTH
    )]
    TooLong,
    #[error("Transaction reference number has invalid format")]
    InvalidFormat,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_transaction_reference_number_write_to() {
        let mut buffer = Cursor::new(Vec::new());
        TransactionReferenceNumber("1234567890".to_string())
            .write_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer.get_ref(), b"1234567890");
    }

    #[test]
    fn test_empty_transaction_reference_number() {
        let result = TransactionReferenceNumber::try_from("");
        assert_eq!(result, Err(TransactionReferenceNumberParseError::Empty));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Transaction reference number cannot be empty"
        );
    }

    #[test]
    fn test_long_transaction_reference_number() {
        let result = TransactionReferenceNumber::try_from(
            "1".repeat(TRANSACTION_REFERENCE_NUMBER_MAX_LENGTH + 1)
                .as_str(),
        );
        assert_eq!(result, Err(TransactionReferenceNumberParseError::TooLong));
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Transaction reference number cannot be longer than {} characters",
                TRANSACTION_REFERENCE_NUMBER_MAX_LENGTH
            )
        );
    }

    #[test]
    fn test_invalid_transaction_reference_number() {
        let result = TransactionReferenceNumber::try_from("/12345678901");
        assert_eq!(
            result,
            Err(TransactionReferenceNumberParseError::InvalidFormat)
        );

        let result = TransactionReferenceNumber::try_from("12345678901/");
        assert_eq!(
            result,
            Err(TransactionReferenceNumberParseError::InvalidFormat)
        );

        let result = TransactionReferenceNumber::try_from("12345678//901");
        assert_eq!(
            result,
            Err(TransactionReferenceNumberParseError::InvalidFormat)
        );
        assert_eq!(
            result.unwrap_err().to_string(),
            "Transaction reference number has invalid format"
        );
    }

    #[test]
    fn test_valid_transaction_reference_number() {
        let result = TransactionReferenceNumber::try_from("1234567890");
        assert_eq!(
            result,
            Ok(TransactionReferenceNumber("1234567890".to_string()))
        );
        assert_eq!(result.unwrap().to_string(), "1234567890");
    }
}
