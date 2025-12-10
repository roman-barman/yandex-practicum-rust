use crate::camt_053_message::statement::credit_debit_identification::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

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

impl Display for TotalEntries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(number) = &self.number {
            writeln!(f, "- Number of entries: {}", number)?;
        }
        if let Some(total_entry_amount) = &self.total_entry_amount {
            writeln!(f, "- Total net entry amount: {}", total_entry_amount)?;
        }
        if let Some(credit_debit_identification) = &self.credit_debit_identification {
            writeln!(
                f,
                "- Credit/debit identification: {}",
                credit_debit_identification
            )?;
        }
        Ok(())
    }
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
