use std::error::Error;
use std::fmt::Display;

const TRANSACTION_REFERENCE_NUMBER_MAX_LENGTH: usize = 16;

#[derive(Debug, PartialEq)]
pub(crate) struct TransactionReferenceNumber(String);

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

        Ok(TransactionReferenceNumber(value.to_string()))
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum TransactionReferenceNumberParseError {
    Empty,
    TooLong,
    InvalidFormat,
}

impl Display for TransactionReferenceNumberParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionReferenceNumberParseError::Empty => {
                write!(f, "Transaction reference number cannot be empty")
            }
            TransactionReferenceNumberParseError::TooLong => write!(
                f,
                "Transaction reference number exceeds {} character length",
                TRANSACTION_REFERENCE_NUMBER_MAX_LENGTH
            ),
            TransactionReferenceNumberParseError::InvalidFormat => {
                write!(f, "Transaction reference number has invalid format")
            }
        }
    }
}

impl Error for TransactionReferenceNumberParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_transaction_reference_number() {
        let result = TransactionReferenceNumber::try_from("");
        assert_eq!(result, Err(TransactionReferenceNumberParseError::Empty));
    }

    #[test]
    fn test_long_transaction_reference_number() {
        let result = TransactionReferenceNumber::try_from("12345678901234567890");
        assert_eq!(result, Err(TransactionReferenceNumberParseError::TooLong));
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
    }

    #[test]
    fn test_valid_transaction_reference_number() {
        let result = TransactionReferenceNumber::try_from("1234567890");
        assert_eq!(
            result,
            Ok(TransactionReferenceNumber("1234567890".to_string()))
        );
    }
}
