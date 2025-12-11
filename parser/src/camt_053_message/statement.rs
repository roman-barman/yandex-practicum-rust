pub(super) mod account;
pub(super) mod account_identification;
pub(super) mod amount;
pub(super) mod balance;
pub(super) mod credit_debit_identification;
pub(super) mod currency;
pub(super) mod date;
pub(super) mod entry;
pub(super) mod financial_institution_identification;
pub(super) mod from_to_date;
pub(super) mod name;
pub(super) mod postal_address;
pub(super) mod sequence_number;
pub(super) mod transactions_summary;

use crate::camt_053_message::creation_date_time::*;
use crate::camt_053_message::identification::*;
use crate::camt_053_message::statement::account::*;
use crate::camt_053_message::statement::balance::*;
use crate::camt_053_message::statement::entry::*;
use crate::camt_053_message::statement::from_to_date::*;
use crate::camt_053_message::statement::sequence_number::*;
use crate::camt_053_message::statement::transactions_summary::*;
use indenter::indented;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Write};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct Statement {
    #[serde(rename = "Id")]
    identification: Identification,
    #[serde(rename = "ElctrncSeqNb")]
    electronic_sequence_number: SequenceNumber,
    #[serde(rename = "LglSeqNb")]
    legal_sequence_number: Option<SequenceNumber>,
    #[serde(rename = "CreDtTm")]
    creation_date_time: CreationDateTime,
    #[serde(rename = "FrToDt")]
    from_to_date: Option<FromToDate>,
    #[serde(rename = "Acct")]
    account: Account,
    #[serde(rename = "Bal")]
    balances: Vec<Balance>,
    #[serde(rename = "TxsSummry")]
    transactions_summary: Option<TransactionsSummary>,
    #[serde(rename = "Ntry")]
    entries: Option<Vec<Entry>>,
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- Identification: {}", self.identification)?;
        writeln!(
            f,
            "- Electronic sequence number: {}",
            self.electronic_sequence_number
        )?;
        if let Some(legal_sequence_number) = &self.legal_sequence_number {
            writeln!(f, "- Legal sequence number: {}", legal_sequence_number)?;
        }
        writeln!(f, "- Creation date time: {}", self.creation_date_time)?;
        if let Some(from_to_date) = &self.from_to_date {
            writeln!(f, "- From to date: {}", from_to_date)?;
        }
        writeln!(f, "- Account")?;
        write!(indented(f), "{}", self.account)?;
        for balance in &self.balances {
            writeln!(f, "- Balance")?;
            write!(indented(f), "{}", balance)?;
        }
        if let Some(transactions_summary) = &self.transactions_summary {
            writeln!(f, "- Transactions summary")?;
            write!(indented(f), "{}", transactions_summary)?;
        }
        if let Some(entries) = &self.entries {
            for entry in entries {
                writeln!(f, "- Entry")?;
                write!(indented(f), "{}", entry)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
impl Statement {
    pub(super) fn new(
        identification: Identification,
        electronic_sequence_number: SequenceNumber,
        legal_sequence_number: Option<SequenceNumber>,
        creation_date_time: CreationDateTime,
        from_to_date: Option<FromToDate>,
        account: Account,
        balances: Vec<Balance>,
        transactions_summary: Option<TransactionsSummary>,
        entries: Option<Vec<Entry>>,
    ) -> Self {
        Self {
            identification,
            electronic_sequence_number,
            legal_sequence_number,
            creation_date_time,
            from_to_date,
            account,
            balances,
            transactions_summary,
            entries,
        }
    }
}
