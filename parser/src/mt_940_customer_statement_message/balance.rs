mod credit_debit_mark;
mod currency_code;
pub(super) mod state;

use crate::mt_940_customer_statement_message::amount::*;
use crate::mt_940_customer_statement_message::balance::credit_debit_mark::*;
use crate::mt_940_customer_statement_message::balance::currency_code::*;
use crate::mt_940_customer_statement_message::balance::state::*;
use crate::mt_940_customer_statement_message::date::*;
use std::any::Any;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;

#[derive(Debug, PartialEq)]
pub(super) struct Balance {
    debit_credit_mark: CreditDebitMark,
    date: Date,
    currency_code: CurrencyCode,
    amount: Amount,
    state: Option<State>,
}

impl TryFrom<&crate::camt_053_message::statement::balance::Balance> for Balance {
    type Error = BalanceParseError;
    fn try_from(
        value: &crate::camt_053_message::statement::balance::Balance,
    ) -> Result<Self, Self::Error> {
        let debit_credit_mark = CreditDebitMark::try_from(value.get_credit_debit_identification())?;
        let date = Date::from(value.get_date());
        let currency_code = CurrencyCode::try_from(value.get_amount().get_currency().as_ref())?;
        let amount = Amount::new(value.get_amount().get_amount());
        Ok(Self {
            debit_credit_mark,
            date,
            currency_code,
            amount,
            state: None,
        })
    }
}

impl Balance {
    pub(super) fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.debit_credit_mark.write_to(writer)?;
        self.date.write_to(writer)?;
        self.currency_code.write_to(writer)?;
        self.amount.write_to(writer)?;
        Ok(())
    }

    pub(super) fn set_state(&mut self, state: State) {
        self.state = Some(state);
    }

    pub(super) fn get_state(&self) -> Option<&State> {
        self.state.as_ref()
    }
}

impl TryFrom<&str> for Balance {
    type Error = BalanceParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(BalanceParseError::Empty);
        }

        let debit_credit_mark = CreditDebitMark::try_from(&value.chars().nth(0).unwrap())?;
        let date = Date::try_from(value.chars().skip(1).take(6).collect::<String>().as_str())?;
        let currency_code =
            CurrencyCode::try_from(value.chars().skip(7).take(3).collect::<String>().as_str())?;
        let amount = Amount::try_from(value.chars().skip(10).collect::<String>().as_str())?;

        Ok(Self {
            debit_credit_mark,
            date,
            currency_code,
            amount,
            state: None,
        })
    }
}

impl Display for Balance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(state) = &self.state {
            writeln!(f, "- State: {}", state)?;
        }
        writeln!(f, "- Debit/Credit: {}", self.debit_credit_mark)?;
        writeln!(f, "- Date: {}", self.date)?;
        writeln!(f, "- Currency code: {}", self.currency_code)?;
        writeln!(f, "- Amount: {}", self.amount)
    }
}

#[derive(Debug)]
pub(super) enum BalanceParseError {
    Empty,
    InvalidFormat(Option<Box<dyn Error>>),
}

impl Display for BalanceParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BalanceParseError::Empty => write!(f, "Opening balance is empty"),
            BalanceParseError::InvalidFormat(None) => {
                write!(f, "Opening balance has invalid format")
            }
            BalanceParseError::InvalidFormat(Some(err)) => {
                write!(f, "Opening balance has invalid format: {}", err)
            }
        }
    }
}

impl From<CreditDebitMarkParseError> for BalanceParseError {
    fn from(value: CreditDebitMarkParseError) -> Self {
        Self::InvalidFormat(Some(Box::new(value)))
    }
}

impl From<CurrencyCodeParseError> for BalanceParseError {
    fn from(value: CurrencyCodeParseError) -> Self {
        Self::InvalidFormat(Some(Box::new(value)))
    }
}

impl From<DateParseError> for BalanceParseError {
    fn from(value: DateParseError) -> Self {
        Self::InvalidFormat(Some(Box::new(value)))
    }
}

impl From<AmountParseError> for BalanceParseError {
    fn from(value: AmountParseError) -> Self {
        Self::InvalidFormat(Some(Box::new(value)))
    }
}

impl PartialEq for BalanceParseError {
    fn eq(&self, other: &Self) -> bool {
        match self {
            BalanceParseError::Empty => matches!(other, BalanceParseError::Empty),
            BalanceParseError::InvalidFormat(None) => {
                matches!(other, BalanceParseError::InvalidFormat(None))
            }
            BalanceParseError::InvalidFormat(Some(err1)) => {
                if let BalanceParseError::InvalidFormat(Some(err2)) = other {
                    (*err1).type_id() == (*err2).type_id()
                } else {
                    false
                }
            }
        }
    }
}

impl Error for BalanceParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use std::io::Cursor;

    #[test]
    fn test_balance_write_to() {
        let mut buffer = Cursor::new(Vec::new());
        Balance::try_from("D230306DKK985623,04")
            .unwrap()
            .write_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer.get_ref(), b"D230306DKK985623,04");
    }

    #[test]
    fn test_empty_opening_balance() {
        let result = Balance::try_from("");
        assert_eq!(result, Err(BalanceParseError::Empty));
        assert_eq!(result.unwrap_err().to_string(), "Opening balance is empty");
    }

    #[test]
    fn test_valid_opening_balance() {
        let result = Balance::try_from("D230306DKK985623,04");
        assert_eq!(
            result,
            Ok(Balance {
                debit_credit_mark: CreditDebitMark::Debit,
                date: Date::new(NaiveDate::from_ymd_opt(2023, 3, 6).unwrap()),
                currency_code: CurrencyCode::try_from("DKK").unwrap(),
                amount: Amount::try_from("985623,04").unwrap(),
                state: None
            })
        );
        assert_eq!(
            result.unwrap().to_string(),
            "- Debit/Credit: Debit\n- Date: 2023-03-06\n- Currency code: DKK\n- Amount: 985623.04\n"
        );

        let mut balance = Balance::try_from("D230306DKK985623,04").unwrap();
        balance.set_state(State::Intermediate);
        assert_eq!(
            balance.to_string(),
            "- State: Intermediate\n- Debit/Credit: Debit\n- Date: 2023-03-06\n- Currency code: DKK\n- Amount: 985623.04\n"
        );
    }
}
