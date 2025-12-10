use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct TotalCreditEntries {
    #[serde(rename = "NbOfNtries")]
    number: Option<usize>,
    #[serde(rename = "Sum")]
    #[serde(with = "rust_decimal::serde::str_option")]
    sum: Option<Decimal>,
}

impl Display for TotalCreditEntries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(number) = &self.number {
            writeln!(f, "- Number of credit entries: {}", number)?;
        }
        if let Some(sum) = &self.sum {
            writeln!(f, "- Total credit amount: {}", sum)?;
        }
        Ok(())
    }
}

impl TotalCreditEntries {
    pub(crate) fn new(number: Option<usize>, sum: Option<Decimal>) -> Self {
        Self { number, sum }
    }
}
