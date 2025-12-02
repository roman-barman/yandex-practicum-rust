use crate::camt_053_message::statement::account::servicer::financial_institution_identification::*;
use serde::{Deserialize, Serialize};

pub(crate) mod financial_institution_identification;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Servicer {
    #[serde(rename = "FinInstnId")]
    identification: FinancialInstitutionIdentification,
}

impl Servicer {
    pub(crate) fn new(identification: FinancialInstitutionIdentification) -> Self {
        Self { identification }
    }
}
