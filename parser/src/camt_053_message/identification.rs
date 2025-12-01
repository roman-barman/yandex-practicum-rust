use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct Identification(String);

impl Identification {
    pub(super) fn new(value: String) -> Self {
        Identification(value)
    }
}
