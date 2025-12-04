use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AccountServicerReference(String);

impl AccountServicerReference {
    pub(crate) fn new(reference: String) -> Self {
        Self(reference)
    }
}
