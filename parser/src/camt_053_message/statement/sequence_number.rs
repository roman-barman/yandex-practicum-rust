use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct SequenceNumber(usize);

impl SequenceNumber {
    pub(crate) fn new(value: usize) -> Self {
        Self(value)
    }
}
