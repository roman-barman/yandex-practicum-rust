use serde::{Deserialize, Serialize};

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

impl Bic {
    pub(crate) fn new(bic: String) -> Self {
        Self(bic)
    }
}
