use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct SequenceNumber(usize);

impl AsRef<usize> for SequenceNumber {
    fn as_ref(&self) -> &usize {
        &self.0
    }
}

impl Display for SequenceNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
impl SequenceNumber {
    pub(crate) fn new(value: usize) -> Self {
        Self(value)
    }
}
