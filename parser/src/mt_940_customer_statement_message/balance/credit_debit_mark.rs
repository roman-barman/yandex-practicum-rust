use crate::camt_053_message::statement::credit_debit_identification::CreditDebitIdentification;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub(super) enum CreditDebitMark {
    Credit,
    Debit,
}

impl CreditDebitMark {
    pub(super) fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            CreditDebitMark::Credit => writer.write_all(b"C"),
            CreditDebitMark::Debit => writer.write_all(b"D"),
        }
    }
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
    use std::io::Cursor;

    #[test]
    fn test_credit_debit_mark_write_to() {
        let mut buffer = Cursor::new(Vec::new());
        CreditDebitMark::Credit.write_to(&mut buffer).unwrap();
        assert_eq!(buffer.get_ref(), b"C");

        let mut buffer = Cursor::new(Vec::new());
        CreditDebitMark::Debit.write_to(&mut buffer).unwrap();
        assert_eq!(buffer.get_ref(), b"D");
    }

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
