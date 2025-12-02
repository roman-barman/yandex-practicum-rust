use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct FinancialInstitutionIdentification {
    #[serde(rename = "BIC")]
    bic: Option<Bic>,
}

impl FinancialInstitutionIdentification {
    pub(crate) fn new(bic: Option<String>) -> Self {
        Self { bic: bic.map(Bic) }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Bic(String);
