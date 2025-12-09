use serde::{Deserialize, Serialize};

use crate::camt_053_message::financial_institution_identification::FinancialInstitutionIdentification;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct RelatedAgents {
    #[serde(rename = "DbtrAgt")]
    debtor_agent: Option<DebtorAgent>,
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

impl DebtorAgent {
    pub(crate) fn new(identification: FinancialInstitutionIdentification) -> Self {
        Self { identification }
    }
}
