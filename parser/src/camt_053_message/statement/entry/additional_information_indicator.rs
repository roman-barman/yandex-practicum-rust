use crate::camt_053_message::identification::Identification;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AdditionalInformationIndicator {
    #[serde(rename = "MsgNmId")]
    message_name_identification: Option<Identification>,
}

impl Display for AdditionalInformationIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(message_name_identification) = &self.message_name_identification {
            writeln!(
                f,
                "- Message name identification: {}",
                message_name_identification
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
impl AdditionalInformationIndicator {
    pub(crate) fn new(message_name_identification: Option<Identification>) -> Self {
        Self {
            message_name_identification,
        }
    }
}
