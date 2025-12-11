use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AccountServicerReference(String);

impl Display for AccountServicerReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
impl AccountServicerReference {
    pub(crate) fn new(reference: String) -> Self {
        Self(reference)
    }
}
