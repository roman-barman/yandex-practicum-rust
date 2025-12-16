use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AccountServicerReference(String);

impl From<&crate::mt_940_customer_statement_message::statement_line::account_owner_ref::AccountOwnerRef> for AccountServicerReference {
    fn from(value: &crate::mt_940_customer_statement_message::statement_line::account_owner_ref::AccountOwnerRef) -> Self {
        AccountServicerReference(value.to_string())
    }
}

impl AsRef<str> for AccountServicerReference {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for AccountServicerReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
impl AccountServicerReference {
    pub(crate) fn new(reference: String) -> Self {
        Self(reference)
    }
}
