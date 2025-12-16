use crate::camt_053_message::identification::*;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(in crate::camt_053_message) enum AccountIdentification {
    #[serde(rename = "IBAN")]
    Iban(Identification),
    #[serde(rename = "Othr")]
    Other(AccountIdentificationOther),
}

impl AccountIdentification {
    pub(crate) fn new_other(id: String) -> AccountIdentification {
        AccountIdentification::Other(AccountIdentificationOther {
            identification: Identification::new(id),
        })
    }
}

impl AsRef<str> for AccountIdentification {
    fn as_ref(&self) -> &str {
        match self {
            AccountIdentification::Iban(iban) => iban.as_ref(),
            AccountIdentification::Other(other) => other.identification.as_ref(),
        }
    }
}

impl Display for AccountIdentification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountIdentification::Iban(iban) => write!(f, "IBAN: {}", iban),
            AccountIdentification::Other(other) => write!(f, "Other: {}", other.identification),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AccountIdentificationOther {
    #[serde(rename = "Id")]
    identification: Identification,
}
