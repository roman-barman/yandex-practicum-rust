pub(crate) mod amount_details;
pub(crate) mod references;
pub(crate) mod related_parties;

use crate::camt_053_message::entry::entry_details::references::References;
use crate::camt_053_message::entry::entry_details::{
    amount_details::AmountDetails, related_parties::RelatedParties,
};
use crate::camt_053_message::financial_institution_identification::FinancialInstitutionIdentification;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct EntryDetails {
    #[serde(rename = "TxDtls")]
    transaction_details: Option<Vec<TransactionDetails>>,
}

impl EntryDetails {
    pub(crate) fn new(transaction_details: Option<Vec<TransactionDetails>>) -> Self {
        Self {
            transaction_details,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct TransactionDetails {
    #[serde(rename = "Refs")]
    references: Option<References>,
    #[serde(rename = "AmtDtls")]
    amount_details: Option<AmountDetails>,
    #[serde(rename = "RltdPties")]
    related_parties: Option<RelatedParties>,
    #[serde(rename = "RltdAgts")]
    related_agents: Option<RelatedAgents>,
    #[serde(rename = "RmtInf")]
    remittance_information: Option<RemittanceInformation>,
    #[serde(rename = "RltdDts")]
    related_dates: Option<RelatedDates>,
    #[serde(rename = "AddtlTxInf")]
    additional_information: Option<String>,
}

impl TransactionDetails {
    pub(crate) fn new(
        references: Option<References>,
        amount_details: Option<AmountDetails>,
        related_parties: Option<RelatedParties>,
        related_agents: Option<RelatedAgents>,
        remittance_information: Option<RemittanceInformation>,
        related_dates: Option<RelatedDates>,
        additional_information: Option<String>,
    ) -> Self {
        Self {
            references,
            amount_details,
            related_parties,
            related_agents,
            remittance_information,
            related_dates,
            additional_information,
        }
    }
}

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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct RemittanceInformation {
    #[serde(rename = "Ustrd")]
    unstructured: Option<Vec<String>>,
}

impl RemittanceInformation {
    pub(crate) fn new(unstructured: Option<Vec<String>>) -> Self {
        Self { unstructured }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct RelatedDates {
    #[serde(rename = "AccptncDtTm")]
    acceptance_date_time: Option<NaiveDateTime>,
}

impl RelatedDates {
    pub(crate) fn new(acceptance_date_time: Option<NaiveDateTime>) -> Self {
        Self {
            acceptance_date_time,
        }
    }
}
