use serde::{Deserialize, Serialize};
use std::fmt::Display;

const CREDIT_IDENTIFICATION: &str = "CRDT";
const DEBIT_IDENTIFICATION: &str = "DBIT";
const UNKNOWN_CREDIT_DEBIT_MARK: &str = "UNKNOWN";

type BalanceCreditDebitMark =
    crate::mt_940_customer_statement_message::balance::credit_debit_mark::CreditDebitMark;
type StatementCreditDebitMark =
    crate::mt_940_customer_statement_message::statement_line::credit_debit_mark::CreditDebitMark;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct CreditDebitIdentification(String);

impl From<&BalanceCreditDebitMark> for CreditDebitIdentification {
    fn from(value: &BalanceCreditDebitMark) -> Self {
        match value {
            BalanceCreditDebitMark::Credit => {
                CreditDebitIdentification(CREDIT_IDENTIFICATION.to_string())
            }
            BalanceCreditDebitMark::Debit => {
                CreditDebitIdentification(DEBIT_IDENTIFICATION.to_string())
            }
        }
    }
}

impl From<&StatementCreditDebitMark> for CreditDebitIdentification {
    fn from(value: &StatementCreditDebitMark) -> Self {
        match value {
            StatementCreditDebitMark::Credit => {
                CreditDebitIdentification(CREDIT_IDENTIFICATION.to_string())
            }
            StatementCreditDebitMark::Debit => {
                CreditDebitIdentification(DEBIT_IDENTIFICATION.to_string())
            }
            _ => CreditDebitIdentification(UNKNOWN_CREDIT_DEBIT_MARK.to_string()),
        }
    }
}

impl Display for CreditDebitIdentification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CreditDebitIdentification {
    pub(crate) fn is_credit(&self) -> bool {
        self.0 == CREDIT_IDENTIFICATION
    }

    pub(crate) fn is_debit(&self) -> bool {
        self.0 == DEBIT_IDENTIFICATION
    }
}

#[cfg(test)]
impl CreditDebitIdentification {
    pub(crate) fn new(value: String) -> Self {
        CreditDebitIdentification(value)
    }
}
