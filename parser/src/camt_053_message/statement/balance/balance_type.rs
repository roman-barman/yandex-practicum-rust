use serde::{Deserialize, Serialize};
use std::fmt::Display;

const OPENING_BALANCE_CODE: &str = "OPBD";
const CLOSING_BALANCE_CODE: &str = "CLBD";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BalanceType {
    #[serde(rename = "CdOrPrtry")]
    code_or_proprietary: CodeOrProprietary,
}

impl BalanceType {
    pub(super) fn new_opening_balance() -> Self {
        BalanceType {
            code_or_proprietary: CodeOrProprietary::Code(OPENING_BALANCE_CODE.to_string()),
        }
    }

    pub(super) fn new_closing_balance() -> Self {
        BalanceType {
            code_or_proprietary: CodeOrProprietary::Code(CLOSING_BALANCE_CODE.to_string()),
        }
    }

    pub(crate) fn is_opening_balance(&self) -> bool {
        self.get_code() == Some(OPENING_BALANCE_CODE)
    }

    pub(crate) fn is_closing_balance(&self) -> bool {
        self.get_code() == Some(CLOSING_BALANCE_CODE)
    }

    pub(crate) fn get_code(&self) -> Option<&str> {
        match &self.code_or_proprietary {
            CodeOrProprietary::Code(code) => Some(code),
            CodeOrProprietary::Proprietary(_) => None,
        }
    }
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
