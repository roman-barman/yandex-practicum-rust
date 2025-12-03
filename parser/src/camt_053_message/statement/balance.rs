use crate::camt_053_message::statement::amount::*;
use crate::camt_053_message::statement::balance::balance_type::*;
use crate::camt_053_message::statement::credit_debit_identification::*;
use crate::camt_053_message::statement::date::*;
use serde::{Deserialize, Serialize};

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
