use serde::{Deserialize, Serialize};

use crate::camt_053_message::identification::Identification;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct References {
    #[serde(rename = "EndToEndId")]
    end_to_end_identification: Option<Identification>,
    #[serde(rename = "TxId")]
    transaction_identification: Option<Identification>,
}

impl References {
    pub(crate) fn new(
        end_to_end_identification: Option<Identification>,
        transaction_identification: Option<Identification>,
    ) -> Self {
        Self {
            end_to_end_identification,
            transaction_identification,
        }
    }
}
