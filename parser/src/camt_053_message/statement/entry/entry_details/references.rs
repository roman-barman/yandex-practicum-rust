use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::camt_053_message::identification::Identification;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct References {
    #[serde(rename = "EndToEndId")]
    end_to_end_identification: Option<Identification>,
    #[serde(rename = "TxId")]
    transaction_identification: Option<Identification>,
}

impl Display for References {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(end_to_end_identification) = &self.end_to_end_identification {
            writeln!(
                f,
                "- End to end identification: {}",
                end_to_end_identification
            )?;
        }
        if let Some(transaction_identification) = &self.transaction_identification {
            writeln!(
                f,
                "- Transaction identification: {}",
                transaction_identification
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
impl References {
    pub(in crate::camt_053_message) fn new(
        end_to_end_identification: Option<Identification>,
        transaction_identification: Option<Identification>,
    ) -> Self {
        Self {
            end_to_end_identification,
            transaction_identification,
        }
    }
}
