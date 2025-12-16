use crate::camt_053_message::creation_date_time::*;
use crate::camt_053_message::identification::Identification;
use crate::mt_940_customer_statement_message::transaction_reference_number::TransactionReferenceNumber;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct GroupHeader {
    #[serde(rename = "MsgId")]
    message_identification: Identification,
    #[serde(rename = "CreDtTm")]
    creation_date_time: CreationDateTime,
}

impl From<&TransactionReferenceNumber> for GroupHeader {
    fn from(value: &TransactionReferenceNumber) -> Self {
        Self {
            message_identification: Identification::new(value.as_ref().to_string()),
            creation_date_time: CreationDateTime::now(),
        }
    }
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

#[cfg(test)]
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
