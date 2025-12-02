use crate::camt_053_message::identification::Identification;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum OwnerIdentification {
    #[serde(rename = "OrgId")]
    Organization(OrganizationIdentification),
    #[serde(rename = "PrvtId")]
    Private,
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum SchemeName {
    #[serde(rename = "Cd")]
    Code(String),
    #[serde(rename = "Prtry")]
    Proprietary(String),
}
