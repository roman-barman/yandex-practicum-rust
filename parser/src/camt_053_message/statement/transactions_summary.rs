use crate::camt_053_message::statement::transactions_summary::total_credit_entries::*;
use crate::camt_053_message::statement::transactions_summary::total_debit_entries::*;
use crate::camt_053_message::statement::transactions_summary::total_entries::*;
use serde::{Deserialize, Serialize};

pub(crate) mod total_credit_entries;
pub(crate) mod total_debit_entries;
pub(crate) mod total_entries;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct TransactionsSummary {
    #[serde(rename = "TtlNtries")]
    total_entries: Option<TotalEntries>,
    #[serde(rename = "TtlCdtNtries")]
    total_credit_entries: Option<TotalCreditEntries>,
    #[serde(rename = "TtlDbtNtries")]
    total_debit_entries: Option<TotalDebitEntries>,
}

impl TransactionsSummary {
    pub(crate) fn new(
        total_entries: Option<TotalEntries>,
        total_credit_entries: Option<TotalCreditEntries>,
        total_debit_entries: Option<TotalDebitEntries>,
    ) -> Self {
        Self {
            total_entries,
            total_credit_entries,
            total_debit_entries,
        }
    }
}
