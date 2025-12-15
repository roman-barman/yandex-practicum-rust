use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct CreditDebitIdentification(String);

impl Display for CreditDebitIdentification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CreditDebitIdentification {
    pub(crate) fn is_credit(&self) -> bool {
        self.0 == "CRDT"
    }

    pub(crate) fn is_debit(&self) -> bool {
        self.0 == "DBIT"
    }
}

#[cfg(test)]
impl CreditDebitIdentification {
    pub fn new(value: String) -> Self {
        CreditDebitIdentification(value)
    }
}
