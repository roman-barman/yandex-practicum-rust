use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Status(String);

impl Status {
    pub(crate) fn new(status: String) -> Self {
        Self(status)
    }
}
