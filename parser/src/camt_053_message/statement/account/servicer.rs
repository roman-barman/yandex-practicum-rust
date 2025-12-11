use crate::camt_053_message::statement::financial_institution_identification::*;
use indenter::indented;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Write};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Servicer {
    #[serde(rename = "FinInstnId")]
    identification: FinancialInstitutionIdentification,
}

impl Display for Servicer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- Financial institution identification")?;
        writeln!(indented(f), "{}", self.identification)
    }
}

#[cfg(test)]
impl Servicer {
    pub(crate) fn new(identification: FinancialInstitutionIdentification) -> Self {
        Self { identification }
    }
}
