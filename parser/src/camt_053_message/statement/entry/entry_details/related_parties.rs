use indenter::indented;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Write};

use crate::camt_053_message::{
    account_identification::AccountIdentification, name::Name, postal_address::PostalAddress,
};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct RelatedParties {
    #[serde(rename = "Dbtr")]
    debtor: Option<Debtor>,
    #[serde(rename = "DbtrAcct")]
    debtor_account: Option<DebtorAccount>,
    #[serde(rename = "Cdtr")]
    creditor: Option<Creditor>,
}

impl Display for RelatedParties {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(debtor) = &self.debtor {
            writeln!(f, "- Debtor")?;
            write!(indented(f), "{}", debtor)?;
        }
        if let Some(debtor_account) = &self.debtor_account {
            writeln!(f, "- Debtor account")?;
            write!(indented(f), "{}", debtor_account)?;
        }
        if let Some(creditor) = &self.creditor {
            writeln!(f, "- Creditor")?;
            write!(indented(f), "{}", creditor)?;
        }
        Ok(())
    }
}

#[cfg(test)]
impl RelatedParties {
    pub(crate) fn new(
        debtor: Option<Debtor>,
        debtor_account: Option<DebtorAccount>,
        creditor: Option<Creditor>,
    ) -> Self {
        Self {
            debtor,
            debtor_account,
            creditor,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Debtor {
    #[serde(rename = "Nm")]
    name: Option<Name>,
    #[serde(rename = "PstlAdr")]
    postal_address: Option<PostalAddress>,
}

impl Display for Debtor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

#[cfg(test)]
impl Debtor {
    pub(crate) fn new(name: Option<Name>, postal_address: Option<PostalAddress>) -> Self {
        Self {
            name,
            postal_address,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct DebtorAccount {
    #[serde(rename = "Id")]
    identification: AccountIdentification,
}

impl Display for DebtorAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- Identification: {}", self.identification)?;
        Ok(())
    }
}

#[cfg(test)]
impl DebtorAccount {
    pub(crate) fn new(identification: AccountIdentification) -> Self {
        Self { identification }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Creditor {
    #[serde(rename = "Nm")]
    name: Option<Name>,
    #[serde(rename = "PstlAdr")]
    postal_address: Option<PostalAddress>,
}

impl Display for Creditor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

#[cfg(test)]
impl Creditor {
    pub(crate) fn new(name: Option<Name>, postal_address: Option<PostalAddress>) -> Self {
        Self {
            name,
            postal_address,
        }
    }
}
