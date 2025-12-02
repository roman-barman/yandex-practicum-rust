use crate::camt_053_message::statement::account::name::*;
use crate::camt_053_message::statement::account::owner::owner_identification::*;
use crate::camt_053_message::statement::account::postal_address::*;
use serde::{Deserialize, Serialize};

pub(crate) mod owner_identification;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Owner {
    #[serde(rename = "Nm")]
    name: Option<Name>,
    #[serde(rename = "PstlAdr")]
    postal_address: Option<PostalAddress>,
    #[serde(rename = "Id")]
    identification: Option<OwnerIdentification>,
}

impl Owner {
    pub(crate) fn new(
        name: Option<Name>,
        postal_address: Option<PostalAddress>,
        identification: Option<OwnerIdentification>,
    ) -> Self {
        Self {
            name,
            postal_address,
            identification,
        }
    }
}
