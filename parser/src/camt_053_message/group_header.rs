use crate::camt_053_message::creation_date_time::*;
use crate::camt_053_message::identification::Identification;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct GroupHeader {
    #[serde(rename = "MsgId")]
    message_identification: Identification,
    #[serde(rename = "CreDtTm")]
    creation_date_time: CreationDateTime,
}

impl Display for GroupHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "- Message identification: {}",
            self.message_identification
        )?;
        writeln!(f, "- Creation date time: {}", self.creation_date_time)?;
        Ok(())
    }
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
