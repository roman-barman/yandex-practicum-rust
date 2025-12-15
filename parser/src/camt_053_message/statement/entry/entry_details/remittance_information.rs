use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct RemittanceInformation {
    #[serde(rename = "Ustrd")]
    unstructured: Option<Vec<String>>,
}

impl RemittanceInformation {
    pub(crate) fn get_unstructured(&self) -> Option<&Vec<String>> {
        self.unstructured.as_ref()
    }
}

impl Display for RemittanceInformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(unstructured) = &self.unstructured {
            for (i, info) in unstructured.iter().enumerate() {
                writeln!(f, "- Unstructured information {}: {}", i + 1, info)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
impl RemittanceInformation {
    pub(crate) fn new(unstructured: Option<Vec<String>>) -> Self {
        Self { unstructured }
    }
}
