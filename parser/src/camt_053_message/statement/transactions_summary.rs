use crate::camt_053_message::statement::transactions_summary::total_credit_entries::*;
use crate::camt_053_message::statement::transactions_summary::total_debit_entries::*;
use crate::camt_053_message::statement::transactions_summary::total_entries::*;
use indenter::indented;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Write};

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

impl Display for TransactionsSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(total_entries) = &self.total_entries {
            writeln!(f, "- Total entries")?;
            write!(indented(f), "{}", total_entries)?;
        }
        if let Some(total_credit_entries) = &self.total_credit_entries {
            writeln!(f, "- Total credit entries")?;
            write!(indented(f), "{}", total_credit_entries)?;
        }
        if let Some(total_debit_entries) = &self.total_debit_entries {
            writeln!(f, "- Total debit entries")?;
            write!(indented(f), "{}", total_debit_entries)?;
        }
        Ok(())
    }
}

#[cfg(test)]
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
