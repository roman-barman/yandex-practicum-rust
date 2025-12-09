use serde::{Deserialize, Serialize};

use crate::camt_053_message::amount::Amount;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct AmountDetails {
    #[serde(rename = "InstdAmt")]
    instructed_amount: Option<InstructedAmount>,
    #[serde(rename = "TxAmt")]
    transaction_amount: Option<TransactionAmount>,
    #[serde(rename = "PrtryAmt")]
    proprietary_amount: Option<Vec<ProprietaryAmount>>,
}

impl AmountDetails {
    pub(crate) fn new(
        instructed_amount: Option<InstructedAmount>,
        transaction_amount: Option<TransactionAmount>,
        proprietary_amount: Option<Vec<ProprietaryAmount>>,
    ) -> Self {
        Self {
            instructed_amount,
            transaction_amount,
            proprietary_amount,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct InstructedAmount {
    #[serde(rename = "Amt")]
    amount: Amount,
}

impl InstructedAmount {
    pub(crate) fn new(amount: Amount) -> Self {
        Self { amount }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct TransactionAmount {
    #[serde(rename = "Amt")]
    amount: Amount,
}

impl TransactionAmount {
    pub(crate) fn new(amount: Amount) -> Self {
        Self { amount }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct ProprietaryAmount {
    #[serde(rename = "Tp")]
    amount_type: String,
    #[serde(rename = "Amt")]
    amount: Amount,
}

impl ProprietaryAmount {
    pub(crate) fn new(amount_type: String, amount: Amount) -> Self {
        Self {
            amount_type,
            amount,
        }
    }
}
