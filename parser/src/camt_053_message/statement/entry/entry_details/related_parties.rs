use serde::{Deserialize, Serialize};

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

impl Creditor {
    pub(crate) fn new(name: Option<Name>, postal_address: Option<PostalAddress>) -> Self {
        Self {
            name,
            postal_address,
        }
    }
}
