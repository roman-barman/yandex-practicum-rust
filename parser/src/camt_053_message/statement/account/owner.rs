use crate::camt_053_message::statement::account::owner::owner_identification::*;
use crate::camt_053_message::statement::name::*;
use crate::camt_053_message::statement::postal_address::*;
use indenter::indented;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Write};

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

impl Display for Owner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            writeln!(f, "- Name: {}", name)?;
        }
        if let Some(postal_address) = &self.postal_address {
            writeln!(f, "- Postal address")?;
            write!(indented(f), "{}", postal_address)?;
        }
        if let Some(identification) = &self.identification {
            writeln!(f, "- Identification")?;
            write!(indented(f), "{}", identification)?;
        }
        Ok(())
    }
}

#[cfg(test)]
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
