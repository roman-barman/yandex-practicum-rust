use crate::mt_940_customer_statement_message::account_identification::AccountIdentificationParseError;
use crate::mt_940_customer_statement_message::balance::BalanceParseError;
use crate::mt_940_customer_statement_message::information_to_account_owner::{
    InformationToAccountOwnerError, InformationToAccountOwnerParseError,
};
use crate::mt_940_customer_statement_message::related_reference::RelatedReferenceParseError;
use crate::mt_940_customer_statement_message::statement_line::supplementary_details::SupplementaryDetailsParseError;
use crate::mt_940_customer_statement_message::statement_line::{
    StatementLineError, StatementLineParseError,
};
use crate::mt_940_customer_statement_message::statement_sequence_number::StatementSequenceNumberParseError;
use crate::mt_940_customer_statement_message::transaction_reference_number::TransactionReferenceNumberParseError;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
/// Error type returned when reading/parsing a SWIFT MT940 customer statement.
///
/// It contains a brief `details` description and may wrap the underlying
/// parsing or I/O error inside `inner` to retain the original cause.
pub struct Mt940CustomerStatementMessageReadError {
    details: String,
    inner: Option<Box<dyn Error>>,
}

impl Mt940CustomerStatementMessageReadError {
    /// Creates an error indicating that the MT940 message has an invalid format.
    pub(super) fn invalid_format() -> Self {
        Self {
            details: "Message has invalid format".to_string(),
            inner: None,
        }
    }

    pub(super) fn unexpected() -> Self {
        Self {
            details: "Unexpected error occurred while parsing MT940 message".to_string(),
            inner: None,
        }
    }

    /// Returns the underlying error cause, if any.
    pub fn inner(&self) -> Option<&dyn Error> {
        self.inner.as_deref()
    }
}

impl Display for Mt940CustomerStatementMessageReadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(inner) = self.inner.as_deref() {
            write!(f, "MT 940 message read error: {}", inner)
        } else {
            write!(f, "MT 940 message read error: {}", self.details)
        }
    }
}

impl Error for Mt940CustomerStatementMessageReadError {}

impl From<std::io::Error> for Mt940CustomerStatementMessageReadError {
    fn from(error: std::io::Error) -> Self {
        Self {
            details: "I/O error".to_string(),
            inner: Some(Box::new(error)),
        }
    }
}

impl From<RelatedReferenceParseError> for Mt940CustomerStatementMessageReadError {
    fn from(error: RelatedReferenceParseError) -> Self {
        Self {
            details: "Related reference parse error".to_string(),
            inner: Some(Box::new(error)),
        }
    }
}

impl From<AccountIdentificationParseError> for Mt940CustomerStatementMessageReadError {
    fn from(error: AccountIdentificationParseError) -> Self {
        Self {
            details: "Account identification parse error".to_string(),
            inner: Some(Box::new(error)),
        }
    }
}

impl From<TransactionReferenceNumberParseError> for Mt940CustomerStatementMessageReadError {
    fn from(error: TransactionReferenceNumberParseError) -> Self {
        Self {
            details: "Transaction reference number parse error".to_string(),
            inner: Some(Box::new(error)),
        }
    }
}

impl From<BalanceParseError> for Mt940CustomerStatementMessageReadError {
    fn from(error: BalanceParseError) -> Self {
        Self {
            details: "Balance parse error".to_string(),
            inner: Some(Box::new(error)),
        }
    }
}

impl From<InformationToAccountOwnerParseError> for Mt940CustomerStatementMessageReadError {
    fn from(error: InformationToAccountOwnerParseError) -> Self {
        Self {
            details: "Information to account owner parse error".to_string(),
            inner: Some(Box::new(error)),
        }
    }
}

impl From<StatementLineParseError> for Mt940CustomerStatementMessageReadError {
    fn from(error: StatementLineParseError) -> Self {
        Self {
            details: "Statement line parse error".to_string(),
            inner: Some(Box::new(error)),
        }
    }
}

impl From<StatementSequenceNumberParseError> for Mt940CustomerStatementMessageReadError {
    fn from(value: StatementSequenceNumberParseError) -> Self {
        Self {
            details: "Statement sequence number parse error".to_string(),
            inner: Some(Box::new(value)),
        }
    }
}

impl From<SupplementaryDetailsParseError> for Mt940CustomerStatementMessageReadError {
    fn from(value: SupplementaryDetailsParseError) -> Self {
        Self {
            details: "Supplementary details parse error".to_string(),
            inner: Some(Box::new(value)),
        }
    }
}

impl From<StatementLineError> for Mt940CustomerStatementMessageReadError {
    fn from(error: StatementLineError) -> Self {
        Self {
            details: "Statement line error".to_string(),
            inner: Some(Box::new(error)),
        }
    }
}

impl From<InformationToAccountOwnerError> for Mt940CustomerStatementMessageReadError {
    fn from(error: InformationToAccountOwnerError) -> Self {
        Self {
            details: "Information to account owner error".to_string(),
            inner: Some(Box::new(error)),
        }
    }
}
