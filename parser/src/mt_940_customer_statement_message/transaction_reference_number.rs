use std::error::Error;
use std::fmt::Display;

const TRANSACTION_REFERENCE_NUMBER_MAX_LENGTH: usize = 16;

#[derive(Debug, PartialEq)]
pub(super) struct TransactionReferenceNumber(String);

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

#[derive(Debug, PartialEq)]
pub(super) enum TransactionReferenceNumberParseError {
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
                "Transaction reference number exceeds {} character length",
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
