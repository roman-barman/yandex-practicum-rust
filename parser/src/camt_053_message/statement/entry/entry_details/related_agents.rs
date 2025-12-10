use indenter::indented;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Write};

use crate::camt_053_message::financial_institution_identification::FinancialInstitutionIdentification;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct RelatedAgents {
    #[serde(rename = "DbtrAgt")]
    debtor_agent: Option<DebtorAgent>,
}

impl Display for RelatedAgents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(debtor_agent) = &self.debtor_agent {
            writeln!(f, "- Debtor agent")?;
            write!(indented(f), "{}", debtor_agent)?;
        }
        Ok(())
    }
}

impl RelatedAgents {
    pub(crate) fn new(debtor_agent: Option<DebtorAgent>) -> Self {
        Self { debtor_agent }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct DebtorAgent {
    #[serde(rename = "FinInstnId")]
    identification: FinancialInstitutionIdentification,
}

impl Display for DebtorAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- Identification")?;
        write!(indented(f), "{}", self.identification)?;
        Ok(())
    }
}

impl DebtorAgent {
    pub(crate) fn new(identification: FinancialInstitutionIdentification) -> Self {
        Self { identification }
    }
}
