use crate::camt_053_message::identification::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum AccountIdentification {
    #[serde(rename = "IBAN")]
    IBAN(Identification),
    #[serde(rename = "Othr")]
    Other(AccountIdentificationOther),
}

impl Display for AccountIdentification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountIdentification::IBAN(iban) => write!(f, "IBAN: {}", iban),
            AccountIdentification::Other(other) => write!(f, "Other: {}", other.identification),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AccountIdentificationOther {
    #[serde(rename = "Id")]
    identification: Identification,
}
