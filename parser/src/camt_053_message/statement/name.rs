use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Name(String);

impl Name {
    pub(crate) fn new(name: String) -> Self {
        Self(name)
    }
}
