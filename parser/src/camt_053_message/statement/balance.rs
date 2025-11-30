use crate::camt_053_message::statement::amount::*;
use crate::camt_053_message::statement::balance::balance_type::*;
use crate::camt_053_message::statement::credit_debit_identification::*;
use crate::camt_053_message::statement::date::*;

mod balance_type;

pub(super) struct Balance {
    balance_type: BalanceType,
    amount: Amount,
    credit_debit_identification: CreditDebitIdentification,
    date: Date,
}
