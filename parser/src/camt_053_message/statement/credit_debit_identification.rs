use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct CreditDebitIdentification(String);

impl CreditDebitIdentification {
    pub fn new(value: String) -> Self {
        CreditDebitIdentification(value)
    }
}
