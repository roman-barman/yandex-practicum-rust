use crate::camt_053_message::statement::amount::*;
use crate::camt_053_message::statement::balance::balance_type::*;
use crate::camt_053_message::statement::credit_debit_identification::*;
use crate::camt_053_message::statement::date::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub(crate) mod balance_type;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Balance {
    #[serde(rename = "Tp")]
    balance_type: BalanceType,
    #[serde(rename = "Amt")]
    amount: Amount,
    #[serde(rename = "CdtDbtInd")]
    credit_debit_identification: CreditDebitIdentification,
    #[serde(rename = "Dt")]
    date: Date,
}

impl Balance {
    pub(crate) fn is_opening_balance(&self) -> bool {
        self.balance_type.is_opening_balance()
    }

    pub(crate) fn is_closing_balance(&self) -> bool {
        self.balance_type.is_closing_balance()
    }

    pub(crate) fn get_credit_debit_identification(&self) -> &CreditDebitIdentification {
        &self.credit_debit_identification
    }

    pub(crate) fn get_date(&self) -> &Date {
        &self.date
    }

    pub(crate) fn get_amount(&self) -> &Amount {
        &self.amount
    }

    pub(super) fn from_opening_balance(
        balance: &crate::mt_940_customer_statement_message::balance::Balance,
    ) -> Self {
        from_balance(BalanceType::new_opening_balance(), balance)
    }

    pub(super) fn from_closing_balance(
        balance: &crate::mt_940_customer_statement_message::balance::Balance,
    ) -> Self {
        from_balance(BalanceType::new_closing_balance(), balance)
    }
}

fn from_balance(
    balance_type: BalanceType,
    balance: &crate::mt_940_customer_statement_message::balance::Balance,
) -> Balance {
    let amount = Amount::from(balance);
    let credit_debit_identification =
        CreditDebitIdentification::from(balance.get_credit_debit_mark());
    let date = Date::from(balance.get_date());
    Balance {
        balance_type,
        amount,
        credit_debit_identification,
        date,
    }
}

impl Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- Balance type: {}", self.balance_type)?;
        writeln!(f, "- Amount: {}", self.amount)?;
        writeln!(
            f,
            "- Credit/debit identification: {}",
            self.credit_debit_identification
        )?;
        writeln!(f, "- Date: {}", self.date)?;
        Ok(())
    }
}

#[cfg(test)]
impl Balance {
    pub(crate) fn new(
        balance_type: BalanceType,
        amount: Amount,
        credit_debit_identification: CreditDebitIdentification,
        date: Date,
    ) -> Self {
        Self {
            balance_type,
            amount,
            credit_debit_identification,
            date,
        }
    }
}
