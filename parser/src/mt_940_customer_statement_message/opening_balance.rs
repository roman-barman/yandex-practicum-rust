use crate::mt_940_customer_statement_message::amount::*;
use crate::mt_940_customer_statement_message::credit_debit_mark::*;
use crate::mt_940_customer_statement_message::currency_code::*;
use crate::mt_940_customer_statement_message::date::*;
use std::any::Any;
use std::error::Error;
use std::fmt::{Display, Formatter};

const OPENING_BALANCE_MAX_LENGTH: usize = 25;
const OPENING_BALANCE_MIN_LENGTH: usize = 12;

#[derive(Debug, PartialEq)]
pub(super) struct OpeningBalance {
    is_intermediate: bool,
    debit_credit_mark: CreditDebitMark,
    date: Date,
    currency_code: CurrencyCode,
    amount: Amount,
}

impl TryFrom<&str> for OpeningBalance {
    type Error = OpeningBalanceParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(OpeningBalanceParseError::Empty);
        }
        if value.len() < OPENING_BALANCE_MIN_LENGTH {
            return Err(OpeningBalanceParseError::InvalidFormat(None));
        }
        if value.len() > OPENING_BALANCE_MAX_LENGTH {
            return Err(OpeningBalanceParseError::TooLong);
        }

        let debit_credit_mark = CreditDebitMark::try_from(&value.chars().nth(0).unwrap())?;
        let date = Date::try_from(value.chars().skip(1).take(6).collect::<String>().as_str())?;
        let currency_code =
            CurrencyCode::try_from(value.chars().skip(7).take(3).collect::<String>().as_str())?;
        let amount = Amount::try_from(value.chars().skip(10).collect::<String>().as_str())?;

        Ok(Self {
            is_intermediate: false,
            debit_credit_mark,
            date,
            currency_code,
            amount,
        })
    }
}

impl Display for OpeningBalance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- Is intermediate: {}", self.is_intermediate)?;
        writeln!(f, "- Debit/Credit: {}", self.debit_credit_mark)?;
        writeln!(f, "- Date: {}", self.date)?;
        writeln!(f, "- Currency code: {}", self.currency_code)?;
        writeln!(f, "- Amount: {}", self.amount)
    }
}

#[derive(Debug)]
pub(super) enum OpeningBalanceParseError {
    Empty,
    TooLong,
    InvalidFormat(Option<Box<dyn Error>>),
}

impl Display for OpeningBalanceParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OpeningBalanceParseError::Empty => write!(f, "Opening balance is empty"),
            OpeningBalanceParseError::TooLong => write!(
                f,
                "Opening balance exceeds {} character length",
                OPENING_BALANCE_MAX_LENGTH
            ),
            OpeningBalanceParseError::InvalidFormat(None) => {
                write!(f, "Opening balance has invalid format")
            }
            OpeningBalanceParseError::InvalidFormat(Some(err)) => {
                write!(f, "Opening balance has invalid format: {}", err)
            }
        }
    }
}

impl<T: Error + 'static> From<T> for OpeningBalanceParseError {
    fn from(value: T) -> Self {
        Self::InvalidFormat(Some(Box::new(value)))
    }
}

impl PartialEq for OpeningBalanceParseError {
    fn eq(&self, other: &Self) -> bool {
        match self {
            OpeningBalanceParseError::Empty => matches!(other, OpeningBalanceParseError::Empty),
            OpeningBalanceParseError::TooLong => matches!(other, OpeningBalanceParseError::TooLong),
            OpeningBalanceParseError::InvalidFormat(None) => {
                matches!(other, OpeningBalanceParseError::InvalidFormat(None))
            }
            OpeningBalanceParseError::InvalidFormat(Some(err1)) => {
                if let OpeningBalanceParseError::InvalidFormat(Some(err2)) = other {
                    err1.type_id() == err2.type_id()
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    #[test]
    fn test_empty_opening_balance() {
        let result = OpeningBalance::try_from("");
        assert_eq!(result, Err(OpeningBalanceParseError::Empty));
        assert_eq!(result.unwrap_err().to_string(), "Opening balance is empty");
    }

    #[test]
    fn test_opening_balance_too_long() {
        let result = OpeningBalance::try_from("1".repeat(OPENING_BALANCE_MAX_LENGTH + 1).as_str());
        assert_eq!(result, Err(OpeningBalanceParseError::TooLong));
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Opening balance exceeds {} character length",
                OPENING_BALANCE_MAX_LENGTH
            )
        );
    }

    #[test]
    fn test_opening_balance_invalid_format() {
        let result = OpeningBalance::try_from("invalid");
        assert_eq!(result, Err(OpeningBalanceParseError::InvalidFormat(None)));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Opening balance has invalid format"
        );
    }

    #[test]
    fn test_valid_opening_balance() {
        let result = OpeningBalance::try_from("D230306DKK985623,04");
        assert_eq!(
            result,
            Ok(OpeningBalance {
                is_intermediate: false,
                debit_credit_mark: CreditDebitMark::Debit,
                date: Date::new(NaiveDate::from_ymd_opt(2023, 3, 6).unwrap()),
                currency_code: CurrencyCode::try_from("DKK").unwrap(),
                amount: Amount::try_from("985623,04").unwrap(),
            })
        );
        assert_eq!(
            result.unwrap().to_string(),
            "- Is intermediate: false\n- Debit/Credit: Debit\n- Date: 2023-03-06\n- Currency code: DKK\n- Amount: 985623.04\n"
        );
    }
}
