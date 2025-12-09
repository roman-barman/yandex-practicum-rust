use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Camt053MessageReadError {
    details: String,
    inner: Option<Box<dyn Error>>,
}

impl Display for Camt053MessageReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CAMT 053 message read error: {}", self.details)
    }
}

impl From<serde_xml_rs::Error> for Camt053MessageReadError {
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

impl Error for Camt053MessageReadError {}
