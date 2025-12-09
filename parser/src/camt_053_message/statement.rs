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
use serde::{Deserialize, Serialize};

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
