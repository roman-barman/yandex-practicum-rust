use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct EntryReference(String);

impl EntryReference {
    pub(crate) fn new(value: String) -> Self {
        Self(value)
    }
}
