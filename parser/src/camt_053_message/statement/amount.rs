use crate::camt_053_message::statement::currency::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Amount {
    #[serde(rename = "@Ccy")]
    currency: Currency,
    #[serde(rename = "#text")]
    amount: Decimal,
}

impl Amount {
    pub(crate) fn new(currency: Currency, amount: Decimal) -> Self {
        Amount { currency, amount }
    }
}
