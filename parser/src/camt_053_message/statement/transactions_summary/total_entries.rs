use crate::camt_053_message::statement::credit_debit_identification::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct TotalEntries {
    #[serde(rename = "NbOfNtries")]
    number: Option<usize>,
    #[serde(rename = "TtlNetNtryAmt")]
    #[serde(with = "rust_decimal::serde::str_option")]
    total_entry_amount: Option<Decimal>,
    #[serde(rename = "CdtDbtInd")]
    credit_debit_identification: Option<CreditDebitIdentification>,
}

impl TotalEntries {
    pub(crate) fn new(
        number: Option<usize>,
        total_entry_amount: Option<Decimal>,
        credit_debit_identification: Option<CreditDebitIdentification>,
    ) -> Self {
        Self {
            number,
            total_entry_amount,
            credit_debit_identification,
        }
    }
}
