use crate::camt_053_message::statement::amount::*;
use crate::camt_053_message::statement::credit_debit_identification::*;
use crate::camt_053_message::statement::date::*;
use crate::camt_053_message::statement::entry::account_servicer_reference::*;
use crate::camt_053_message::statement::entry::bank_transaction_code::*;
use crate::camt_053_message::statement::entry::entry_reference::*;
use crate::camt_053_message::statement::entry::status::*;

mod account_servicer_reference;
mod additional_information_indicator;
mod bank_transaction_code;
mod entry_reference;
mod status;

#[derive(Debug, PartialEq)]
pub(super) struct Entry {
    reference: EntryReference,
    amount: Amount,
    credit_debit_identification: CreditDebitIdentification,
    status: Status,
    booking_date: Option<Date>,
    value_date: Option<Date>,
    account_servicer_reference: Option<AccountServicerReference>,
    bank_transaction_code: BankTransactionCode,
}
