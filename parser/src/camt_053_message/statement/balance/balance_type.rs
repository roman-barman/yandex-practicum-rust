use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct BalanceType {
    #[serde(rename = "CdOrPrtry")]
    code_or_proprietary: CodeOrProprietary,
}

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
