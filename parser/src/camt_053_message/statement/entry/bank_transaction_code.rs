use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BankTransactionCode {
    #[serde(rename = "Domn")]
    domain: Option<Domain>,
    #[serde(rename = "Prtry")]
    proprietary: Option<Proprietary>,
}

impl BankTransactionCode {
    pub(crate) fn new(domain: Option<Domain>, proprietary: Option<Proprietary>) -> Self {
        Self {
            domain,
            proprietary,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Domain {
    #[serde(rename = "Cd")]
    code: String,
    #[serde(rename = "Fmly")]
    family: Family,
}

impl Domain {
    pub(crate) fn new(code: String, family: Family) -> Self {
        Self { code, family }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Family {
    #[serde(rename = "Cd")]
    code: String,
    #[serde(rename = "SubFmlyCd")]
    sub_family_code: String,
}

impl Family {
    pub(crate) fn new(code: String, sub_family_code: String) -> Self {
        Self {
            code,
            sub_family_code,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Proprietary {
    #[serde(rename = "Cd")]
    code: String,
    #[serde(rename = "Issr")]
    issuer: Option<String>,
}

impl Proprietary {
    pub(crate) fn new(code: String, issuer: Option<String>) -> Self {
        Self { code, issuer }
    }
}
