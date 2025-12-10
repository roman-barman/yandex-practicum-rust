use indenter::indented;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Write};

use crate::camt_053_message::{name::Name, postal_address::PostalAddress};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct FinancialInstitutionIdentification {
    #[serde(rename = "BIC")]
    bic: Option<Bic>,
    #[serde(rename = "Nm")]
    name: Option<Name>,
    #[serde(rename = "PstlAdr")]
    postal_address: Option<PostalAddress>,
}

impl Display for FinancialInstitutionIdentification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(bic) = &self.bic {
            writeln!(f, "- BIC: {}", bic)?;
        }
        if let Some(name) = &self.name {
            writeln!(f, "- Name: {}", name)?;
        }
        if let Some(postal_address) = &self.postal_address {
            writeln!(f, "- Postal address")?;
            write!(indented(f), "{}", postal_address)?;
        }
        Ok(())
    }
}

impl FinancialInstitutionIdentification {
    pub(crate) fn new(
        bic: Option<Bic>,
        name: Option<Name>,
        postal_address: Option<PostalAddress>,
    ) -> Self {
        Self {
            bic,
            name,
            postal_address,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Bic(String);

impl Display for Bic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Bic {
    pub(crate) fn new(bic: String) -> Self {
        Self(bic)
    }
}
