use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct TotalDebitEntries {
    #[serde(rename = "NbOfNtries")]
    number: Option<usize>,
    #[serde(rename = "Sum")]
    #[serde(with = "rust_decimal::serde::str_option")]
    sum: Option<Decimal>,
}

impl TotalDebitEntries {
    pub(crate) fn new(number: Option<usize>, sum: Option<Decimal>) -> Self {
        Self { number, sum }
    }
}
