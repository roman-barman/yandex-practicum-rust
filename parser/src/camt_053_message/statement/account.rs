use crate::Mt940CustomerStatementMessage;
use crate::camt_053_message::statement::account::owner::*;
use crate::camt_053_message::statement::account::servicer::*;
use crate::camt_053_message::statement::account_identification::*;
use crate::camt_053_message::statement::currency::*;
use crate::camt_053_message::statement::name::*;
use indenter::indented;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Write};

pub(crate) mod owner;
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

impl From<&Mt940CustomerStatementMessage> for Account {
    fn from(value: &Mt940CustomerStatementMessage) -> Self {
        let identification =
            AccountIdentification::new_other(value.get_account_identification().to_string());
        let currency = Currency::new(value.get_opening_balance().get_currency_code().to_string());
        Self {
            identification,
            currency,
            name: None,
            owner: None,
            servicer: None,
        }
    }
}

impl Account {
    pub(crate) fn get_identification(&self) -> &str {
        self.identification.as_ref()
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- Identification: {}", self.identification)?;
        writeln!(f, "- Currency: {}", self.currency)?;
        if let Some(name) = &self.name {
            writeln!(f, "- Name: {}", name)?;
        }
        if let Some(owner) = &self.owner {
            writeln!(f, "- Owner")?;
            write!(indented(f), "{}", owner)?;
        }
        if let Some(servicer) = &self.servicer {
            writeln!(f, "- Servicer")?;
            write!(indented(f), "{}", servicer)?;
        }
        Ok(())
    }
}

#[cfg(test)]
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
