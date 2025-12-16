use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Camt053MessageError {
    details: String,
    inner: Option<Box<dyn Error>>,
}

/// An error that can occur when reading or writing a CAMT.053 message.
///
/// The error keeps a human‑readable `details` message and may wrap
/// an underlying error in `inner` for additional context.
impl Display for Camt053MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(inner) = self.inner.as_deref() {
            write!(f, "CAMT 053 message error: {}", inner)
        } else {
            write!(f, "CAMT 053 message error: {}", self.details)
        }
    }
}

impl From<serde_xml_rs::Error> for Camt053MessageError {
    fn from(value: serde_xml_rs::Error) -> Self {
        match value {
            serde_xml_rs::Error::Unsupported(_) => Self {
                details: "Unexpected error".to_string(),
                inner: Some(Box::new(value)),
            },
            _ => Self {
                details: "Invalid format".to_string(),
                inner: Some(Box::new(value)),
            },
        }
    }
}

impl Error for Camt053MessageError {}
