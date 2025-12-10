use crate::camt_053_message::identification::Identification;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum OwnerIdentification {
    #[serde(rename = "OrgId")]
    Organization(OrganizationIdentification),
    #[serde(rename = "PrvtId")]
    Private,
}

impl Display for OwnerIdentification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OwnerIdentification::Organization(org) => writeln!(f, "Organization: {}", org),
            OwnerIdentification::Private => writeln!(f, "Private"),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum OrganizationIdentification {
    #[serde(rename = "BICOrBEI")]
    BicOrBei(String),
    #[serde(rename = "Othr")]
    Other {
        #[serde(rename = "Id")]
        id: Identification,
        #[serde(rename = "SchmeNm")]
        scheme_name: Option<SchemeName>,
    },
}

impl Display for OrganizationIdentification {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            OrganizationIdentification::BicOrBei(bic) => write!(f, "BIC or BEI: {}", bic),
            OrganizationIdentification::Other { id, scheme_name } => {
                write!(f, "Other: {}", id)?;
                if let Some(name) = scheme_name {
                    write!(f, ", {}", name)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum SchemeName {
    #[serde(rename = "Cd")]
    Code(String),
    #[serde(rename = "Prtry")]
    Proprietary(String),
}

impl Display for SchemeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemeName::Code(code) => write!(f, "Code: {}", code),
            SchemeName::Proprietary(prop) => write!(f, "Proprietary: {}", prop),
        }
    }
}
