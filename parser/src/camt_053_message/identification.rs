use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct Identification(String);

impl Display for Identification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Identification {
    pub(super) fn new(value: String) -> Self {
        Identification(value)
    }
}
