use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct EntryReference(String);

impl Display for EntryReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
impl EntryReference {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }
}
