pub(crate) mod amount_details;
pub(crate) mod references;
pub(crate) mod related_agents;
pub(crate) mod related_dates;
pub(crate) mod related_parties;
pub(crate) mod remittance_information;

use crate::camt_053_message::entry::entry_details::references::References;
use crate::camt_053_message::entry::entry_details::related_agents::RelatedAgents;
use crate::camt_053_message::entry::entry_details::related_dates::RelatedDates;
use crate::camt_053_message::entry::entry_details::remittance_information::RemittanceInformation;
use crate::camt_053_message::entry::entry_details::{
    amount_details::AmountDetails, related_parties::RelatedParties,
};
use indenter::indented;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Write};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct EntryDetails {
    #[serde(rename = "TxDtls")]
    transaction_details: Option<Vec<TransactionDetails>>,
}

impl Display for EntryDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(transaction_details) = &self.transaction_details {
            for transaction_detail in transaction_details {
                writeln!(f, "- Transaction detail")?;
                write!(indented(f), "{}", transaction_detail)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
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

impl Display for TransactionDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(references) = &self.references {
            writeln!(f, "- References")?;
            write!(indented(f), "{}", references)?;
        }
        if let Some(amount_details) = &self.amount_details {
            writeln!(f, "- Amount details")?;
            write!(indented(f), "{}", amount_details)?;
        }
        if let Some(related_parties) = &self.related_parties {
            writeln!(f, "- Related parties")?;
            write!(indented(f), "{}", related_parties)?;
        }
        if let Some(related_agents) = &self.related_agents {
            writeln!(f, "- Related agents")?;
            write!(indented(f), "{}", related_agents)?;
        }
        if let Some(remittance_information) = &self.remittance_information {
            writeln!(f, "- Remittance information")?;
            write!(indented(f), "{}", remittance_information)?;
        }
        if let Some(related_dates) = &self.related_dates {
            writeln!(f, "- Related dates")?;
            write!(indented(f), "{}", related_dates)?;
        }
        if let Some(additional_information) = &self.additional_information {
            writeln!(f, "- Additional information: {}", additional_information)?;
        }
        Ok(())
    }
}

#[cfg(test)]
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
