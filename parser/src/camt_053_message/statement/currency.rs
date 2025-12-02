use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Currency(String);

impl Currency {
    pub(crate) fn new(currency_code: String) -> Self {
        Self(currency_code)
    }
}
