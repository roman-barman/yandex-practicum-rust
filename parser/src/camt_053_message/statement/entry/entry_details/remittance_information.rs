use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct RemittanceInformation {
    #[serde(rename = "Ustrd")]
    unstructured: Option<Vec<String>>,
}

impl RemittanceInformation {
    pub(crate) fn new(unstructured: Option<Vec<String>>) -> Self {
        Self { unstructured }
    }
}
