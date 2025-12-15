use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(in crate::camt_053_message) struct Identification(String);

impl Display for Identification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Identification {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
impl Identification {
    pub(super) fn new(value: String) -> Self {
        Identification(value)
    }
}
