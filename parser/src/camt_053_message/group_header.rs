use crate::camt_053_message::creation_date_time::*;
use crate::camt_053_message::identification::Identification;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct GroupHeader {
    #[serde(rename = "MsgId")]
    message_identification: Identification,
    #[serde(rename = "CreDtTm")]
    creation_date_time: CreationDateTime,
}

impl GroupHeader {
    pub(super) fn new(
        message_identification: Identification,
        creation_date_time: CreationDateTime,
    ) -> Self {
        GroupHeader {
            message_identification,
            creation_date_time,
        }
    }
}
