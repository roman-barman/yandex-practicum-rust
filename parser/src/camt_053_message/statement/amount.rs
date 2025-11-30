use crate::camt_053_message::statement::currency::*;
use rust_decimal::Decimal;

#[derive(Debug, PartialEq)]
pub(super) struct Amount {
    currency: Currency,
    amount: Decimal,
}
