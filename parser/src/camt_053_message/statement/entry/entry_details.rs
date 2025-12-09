use crate::camt_053_message::account_identification::AccountIdentification;
use crate::camt_053_message::financial_institution_identification::FinancialInstitutionIdentification;
use crate::camt_053_message::name::Name;
use crate::camt_053_message::statement::amount::Amount;
use crate::camt_053_message::{identification::Identification, postal_address::PostalAddress};
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
pub(crate) struct References {
    #[serde(rename = "EndToEndId")]
    end_to_end_identification: Option<Identification>,
    #[serde(rename = "TxId")]
    transaction_identification: Option<Identification>,
}

impl References {
    pub(crate) fn new(
        end_to_end_identification: Option<Identification>,
        transaction_identification: Option<Identification>,
    ) -> Self {
        Self {
            end_to_end_identification,
            transaction_identification,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AmountDetails {
    #[serde(rename = "InstdAmt")]
    instructed_amount: Option<InstructedAmount>,
    #[serde(rename = "TxAmt")]
    transaction_amount: Option<TransactionAmount>,
    #[serde(rename = "PrtryAmt")]
    proprietary_amount: Option<Vec<ProprietaryAmount>>,
}

impl AmountDetails {
    pub(crate) fn new(
        instructed_amount: Option<InstructedAmount>,
        transaction_amount: Option<TransactionAmount>,
        proprietary_amount: Option<Vec<ProprietaryAmount>>,
    ) -> Self {
        Self {
            instructed_amount,
            transaction_amount,
            proprietary_amount,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct InstructedAmount {
    #[serde(rename = "Amt")]
    amount: Amount,
}

impl InstructedAmount {
    pub(crate) fn new(amount: Amount) -> Self {
        Self { amount }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct TransactionAmount {
    #[serde(rename = "Amt")]
    amount: Amount,
}

impl TransactionAmount {
    pub(crate) fn new(amount: Amount) -> Self {
        Self { amount }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ProprietaryAmount {
    #[serde(rename = "Tp")]
    amount_type: String,
    #[serde(rename = "Amt")]
    amount: Amount,
}

impl ProprietaryAmount {
    pub(crate) fn new(amount_type: String, amount: Amount) -> Self {
        Self {
            amount_type,
            amount,
        }
    }
}

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
