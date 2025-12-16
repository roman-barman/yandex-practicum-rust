use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Currency(String);

impl AsRef<str> for Currency {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Currency {
    pub(crate) fn new(currency_code: String) -> Self {
        Self(currency_code)
    }
}
