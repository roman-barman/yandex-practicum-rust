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

impl From<&crate::mt_940_customer_statement_message::balance::Balance> for Amount {
    fn from(value: &crate::mt_940_customer_statement_message::balance::Balance) -> Self {
        Amount {
            currency: Currency::new(value.get_currency_code().to_string()),
            amount: *value.get_amount().as_ref(),
        }
    }
}

impl Amount {
    pub(crate) fn get_currency(&self) -> &Currency {
        &self.currency
    }

    pub(crate) fn get_amount(&self) -> Decimal {
        self.amount
    }

    pub(crate) fn new(currency: Currency, amount: Decimal) -> Self {
        Amount { currency, amount }
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.amount, self.currency)
    }
}
