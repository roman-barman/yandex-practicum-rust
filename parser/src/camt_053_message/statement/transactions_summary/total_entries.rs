use crate::camt_053_message::statement::credit_debit_identification::*;
use rust_decimal::Decimal;

#[derive(Debug, PartialEq)]
pub(super) struct TotalEntries {
    number: Option<usize>,
    total_entry_amount: Option<Decimal>,
    credit_debit_identification: Option<CreditDebitIdentification>,
}
