use crate::camt_053_message::statement::transactions_summary::total_credit_entries::*;
use crate::camt_053_message::statement::transactions_summary::total_debit_entries::*;
use crate::camt_053_message::statement::transactions_summary::total_entries::*;

mod total_credit_entries;
mod total_debit_entries;
mod total_entries;

pub(super) struct TransactionsSummary {
    total_entries: Option<TotalEntries>,
    total_credit_entries: Option<TotalCreditEntries>,
    total_debit_entries: Option<TotalDebitEntries>,
}
