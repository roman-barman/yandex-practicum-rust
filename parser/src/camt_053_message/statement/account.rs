use crate::camt_053_message::statement::account::account_identification::*;
use crate::camt_053_message::statement::account::name::*;
use crate::camt_053_message::statement::account::owner::*;
use crate::camt_053_message::statement::account::servicer::*;
use crate::camt_053_message::statement::currency::*;
use serde::{Deserialize, Serialize};

pub(crate) mod account_identification;
pub(crate) mod name;
pub(crate) mod owner;
pub(crate) mod postal_address;
pub(crate) mod servicer;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Account {
    #[serde(rename = "Id")]
    identification: AccountIdentification,
    #[serde(rename = "Ccy")]
    currency: Currency,
    #[serde(rename = "Nm")]
    name: Option<Name>,
    #[serde(rename = "Ownr")]
    owner: Option<Owner>,
    #[serde(rename = "Svcr")]
    servicer: Option<Servicer>,
}

impl Account {
    pub fn new(
        identification: AccountIdentification,
        currency: Currency,
        name: Option<Name>,
        owner: Option<Owner>,
        servicer: Option<Servicer>,
    ) -> Self {
        Self {
            identification,
            currency,
            name,
            owner,
            servicer,
        }
    }
}
