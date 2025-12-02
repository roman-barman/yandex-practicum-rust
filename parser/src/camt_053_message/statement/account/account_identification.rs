use crate::camt_053_message::identification::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum AccountIdentification {
    #[serde(rename = "IBAN")]
    IBAN(Identification),
    #[serde(rename = "Othr")]
    Other,
}
