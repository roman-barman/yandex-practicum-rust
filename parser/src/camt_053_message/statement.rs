mod account;
mod amount;
mod balance;
mod credit_debit_identification;
mod currency;
mod date;
mod entry;
mod from_to_date;
pub(super) mod sequence_number;
mod transactions_summary;

use crate::camt_053_message::creation_date_time::*;
use crate::camt_053_message::identification::*;
use crate::camt_053_message::statement::account::*;
use crate::camt_053_message::statement::balance::*;
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
    //creation_date_time: CreationDateTime,
    //from_to_date: Option<FromToDate>,
    //account: Account,
    //balances: Vec<Balance>,
    //transactions_summary: Option<TransactionsSummary>,
}

impl Statement {
    pub(super) fn new(
        identification: Identification,
        electronic_sequence_number: SequenceNumber,
        legal_sequence_number: Option<SequenceNumber>,
    ) -> Self {
        Self {
            identification,
            electronic_sequence_number,
            legal_sequence_number,
        }
    }
}
