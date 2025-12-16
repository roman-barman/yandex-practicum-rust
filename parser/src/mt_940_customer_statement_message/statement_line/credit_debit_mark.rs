use crate::camt_053_message::statement::credit_debit_identification::CreditDebitIdentification;
use std::error::Error;
use std::fmt::{Display, Formatter};

pub(super) const CREDIT_DEBIT_MARK_MAX_LENGTH: usize = 2;

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum CreditDebitMark {
    Credit,
    Debit,
    ReversalOfCredit,
    ReversalOfDebit,
}

impl CreditDebitMark {
    pub(super) fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            CreditDebitMark::Credit => writer.write_all(b"C"),
            CreditDebitMark::Debit => writer.write_all(b"D"),
            CreditDebitMark::ReversalOfCredit => writer.write_all(b"RC"),
            CreditDebitMark::ReversalOfDebit => writer.write_all(b"RD"),
        }
    }
}

impl TryFrom<&CreditDebitIdentification> for CreditDebitMark {
    type Error = CreditDebitMarkParseError;
    fn try_from(value: &CreditDebitIdentification) -> Result<Self, Self::Error> {
        if value.is_credit() {
            Ok(Self::Credit)
        } else if value.is_debit() {
            Ok(Self::Debit)
        } else {
            Err(CreditDebitMarkParseError::InvalidValue)
        }
    }
}

impl TryFrom<&str> for CreditDebitMark {
    type Error = CreditDebitMarkParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(CreditDebitMarkParseError::Empty);
        }
        if value.len() > CREDIT_DEBIT_MARK_MAX_LENGTH {
            return Err(CreditDebitMarkParseError::TooLong);
        }

        match value {
            "C" => Ok(Self::Credit),
            "D" => Ok(Self::Debit),
            "RC" => Ok(Self::ReversalOfCredit),
            "RD" => Ok(Self::ReversalOfDebit),
            _ => Err(CreditDebitMarkParseError::InvalidValue),
        }
    }
}

impl Display for CreditDebitMark {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CreditDebitMark::Credit => write!(f, "Credit"),
            CreditDebitMark::Debit => write!(f, "Debit"),
            CreditDebitMark::ReversalOfCredit => write!(f, "Reversal of Credit"),
            CreditDebitMark::ReversalOfDebit => write!(f, "Reversal of Debit"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum CreditDebitMarkParseError {
    Empty,
    TooLong,
    InvalidValue,
}

impl Display for CreditDebitMarkParseError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            CreditDebitMarkParseError::InvalidValue => {
                write!(f, "Invalid credit/debit mark")
            }
            CreditDebitMarkParseError::Empty => {
                write!(f, "Credit/debit mark cannot be empty")
            }
            CreditDebitMarkParseError::TooLong => {
                write!(
                    f,
                    "Credit/debit mark cannot be longer than {} characters",
                    CREDIT_DEBIT_MARK_MAX_LENGTH
                )
            }
        }
    }
}

impl Error for CreditDebitMarkParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_credit_debit_mark_write_to() {
        let mut buffer = Cursor::new(Vec::new());
        CreditDebitMark::Credit.write_to(&mut buffer).unwrap();
        assert_eq!(buffer.get_ref(), b"C");

        let mut buffer = Cursor::new(Vec::new());
        CreditDebitMark::Debit.write_to(&mut buffer).unwrap();
        assert_eq!(buffer.get_ref(), b"D");

        let mut buffer = Cursor::new(Vec::new());
        CreditDebitMark::ReversalOfCredit
            .write_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer.get_ref(), b"RC");

        let mut buffer = Cursor::new(Vec::new());
        CreditDebitMark::ReversalOfDebit
            .write_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer.get_ref(), b"RD");
    }

    #[test]
    fn test_empty_credit_debit_mark() {
        let result = CreditDebitMark::try_from("");
        assert_eq!(result, Err(CreditDebitMarkParseError::Empty));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Credit/debit mark cannot be empty"
        );
    }

    #[test]
    fn test_too_long_credit_debit_mark() {
        let result = CreditDebitMark::try_from("CDE");
        assert_eq!(result, Err(CreditDebitMarkParseError::TooLong));
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Credit/debit mark cannot be longer than {} characters",
                CREDIT_DEBIT_MARK_MAX_LENGTH
            )
        );
    }

    #[test]
    fn test_invalid_credit_debit_mark() {
        let result = CreditDebitMark::try_from("X");
        assert_eq!(result, Err(CreditDebitMarkParseError::InvalidValue));
        assert_eq!(result.unwrap_err().to_string(), "Invalid credit/debit mark");
    }

    #[test]
    fn test_valid_credit_debit_mark() {
        let result = CreditDebitMark::try_from("C");
        assert_eq!(result, Ok(CreditDebitMark::Credit));
        assert_eq!(result.unwrap().to_string(), "Credit");

        let result = CreditDebitMark::try_from("D");
        assert_eq!(result, Ok(CreditDebitMark::Debit));
        assert_eq!(result.unwrap().to_string(), "Debit");

        let result = CreditDebitMark::try_from("RC");
        assert_eq!(result, Ok(CreditDebitMark::ReversalOfCredit));
        assert_eq!(result.unwrap().to_string(), "Reversal of Credit");

        let result = CreditDebitMark::try_from("RD");
        assert_eq!(result, Ok(CreditDebitMark::ReversalOfDebit));
        assert_eq!(result.unwrap().to_string(), "Reversal of Debit");
    }
}
