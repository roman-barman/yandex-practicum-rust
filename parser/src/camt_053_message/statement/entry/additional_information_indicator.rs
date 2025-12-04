use crate::camt_053_message::identification::Identification;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AdditionalInformationIndicator {
    #[serde(rename = "MsgNmId")]
    message_name_identification: Option<Identification>,
}

impl AdditionalInformationIndicator {
    pub(crate) fn new(message_name_identification: Option<Identification>) -> Self {
        Self {
            message_name_identification,
        }
    }
}
