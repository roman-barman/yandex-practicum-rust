use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub(super) enum CreditDebitMark {
    Credit,
    Debit,
}

impl TryFrom<&char> for CreditDebitMark {
    type Error = CreditDebitMarkParseError;
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            'C' => Ok(Self::Credit),
            'D' => Ok(Self::Debit),
            _ => Err(CreditDebitMarkParseError::InvalidValue),
        }
    }
}

impl Display for CreditDebitMark {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CreditDebitMark::Credit => write!(f, "Credit"),
            CreditDebitMark::Debit => write!(f, "Debit"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum CreditDebitMarkParseError {
    InvalidValue,
}

impl std::fmt::Display for CreditDebitMarkParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CreditDebitMarkParseError::InvalidValue => {
                write!(f, "Invalid credit/debit mark")
            }
        }
    }
}

impl Error for CreditDebitMarkParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_credit_debit_mark() {
        let result = CreditDebitMark::try_from(&'X');
        assert_eq!(result, Err(CreditDebitMarkParseError::InvalidValue));
        assert_eq!(result.unwrap_err().to_string(), "Invalid credit/debit mark");
    }

    #[test]
    fn test_valid_credit_debit_mark() {
        let result = CreditDebitMark::try_from(&'C');
        assert_eq!(result, Ok(CreditDebitMark::Credit));
        assert_eq!(result.unwrap().to_string(), "Credit");

        let result = CreditDebitMark::try_from(&'D');
        assert_eq!(result, Ok(CreditDebitMark::Debit));
        assert_eq!(result.unwrap().to_string(), "Debit");
    }
}
