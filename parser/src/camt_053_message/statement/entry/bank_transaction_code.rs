use indenter::indented;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Write};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BankTransactionCode {
    #[serde(rename = "Domn")]
    domain: Option<Domain>,
    #[serde(rename = "Prtry")]
    proprietary: Option<Proprietary>,
}

impl Display for BankTransactionCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(domain) = &self.domain {
            writeln!(f, "- Domain")?;
            write!(indented(f), "{}", domain)?;
        }
        if let Some(proprietary) = &self.proprietary {
            writeln!(f, "- Proprietary")?;
            write!(indented(f), "{}", proprietary)?;
        }
        Ok(())
    }
}

#[cfg(test)]
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

impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- Code: {}", self.code)?;
        writeln!(f, "- Family")?;
        write!(indented(f), "{}", self.family)?;
        Ok(())
    }
}

#[cfg(test)]
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

impl Display for Family {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- Family code: {}", self.code)?;
        writeln!(f, "- Sub-family code: {}", self.sub_family_code)?;
        Ok(())
    }
}

#[cfg(test)]
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

impl Display for Proprietary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- Proprietary code: {}", self.code)?;
        if let Some(issuer) = &self.issuer {
            writeln!(f, "- Issuer: {}", issuer)?;
        }
        Ok(())
    }
}

#[cfg(test)]
impl Proprietary {
    pub(crate) fn new(code: String, issuer: Option<String>) -> Self {
        Self { code, issuer }
    }
}
