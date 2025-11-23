use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub(super) enum TransactionType {
    SwiftTransfer,
    NonSwiftTransfer,
    FirstAdvice,
}

impl TryFrom<&char> for TransactionType {
    type Error = TransactionTypeParseError;
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::SwiftTransfer),
            'N' => Ok(Self::NonSwiftTransfer),
            'F' => Ok(Self::FirstAdvice),
            _ => Err(TransactionTypeParseError::InvalidValue),
        }
    }
}

impl Display for TransactionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SwiftTransfer => write!(f, "SWIFT transfer"),
            Self::NonSwiftTransfer => write!(f, "Non-SWIFT transfer"),
            Self::FirstAdvice => write!(f, "First advice"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum TransactionTypeParseError {
    InvalidValue,
}

impl Display for TransactionTypeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidValue => write!(f, "Invalid transaction type value"),
        }
    }
}

impl std::error::Error for TransactionTypeParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_transaction_type() {
        let result = TransactionType::try_from(&'A');
        assert_eq!(result, Err(TransactionTypeParseError::InvalidValue));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid transaction type value"
        );
    }

    #[test]
    fn test_valid_transaction_type() {
        let result = TransactionType::try_from(&'S');
        assert_eq!(result, Ok(TransactionType::SwiftTransfer));
        assert_eq!(result.unwrap().to_string(), "SWIFT transfer");

        let result = TransactionType::try_from(&'F');
        assert_eq!(result, Ok(TransactionType::FirstAdvice));
        assert_eq!(result.unwrap().to_string(), "First advice");

        let result = TransactionType::try_from(&'N');
        assert_eq!(result, Ok(TransactionType::NonSwiftTransfer));
        assert_eq!(result.unwrap().to_string(), "Non-SWIFT transfer");
    }
}
