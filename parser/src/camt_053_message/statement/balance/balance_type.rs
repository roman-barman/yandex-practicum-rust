use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BalanceType {
    #[serde(rename = "CdOrPrtry")]
    code_or_proprietary: CodeOrProprietary,
}

impl Display for BalanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code_or_proprietary)
    }
}

#[cfg(test)]
impl BalanceType {
    pub(crate) fn new(code_or_proprietary: CodeOrProprietary) -> Self {
        BalanceType {
            code_or_proprietary,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum CodeOrProprietary {
    #[serde(rename = "Cd")]
    Code(String),
    #[serde(rename = "Prtry")]
    Proprietary(String),
}

impl Display for CodeOrProprietary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodeOrProprietary::Code(code) => write!(f, "Code: {}", code),
            CodeOrProprietary::Proprietary(prop) => write!(f, "Proprietary: {}", prop),
        }
    }
}
