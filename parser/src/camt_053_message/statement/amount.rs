use crate::camt_053_message::statement::currency::*;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Amount {
    #[serde(rename = "@Ccy")]
    currency: Currency,
    #[serde(rename = "#text")]
    amount: Decimal,
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.amount, self.currency)
    }
}

#[cfg(test)]
impl Amount {
    pub(crate) fn new(currency: Currency, amount: Decimal) -> Self {
        Amount { currency, amount }
    }
}
