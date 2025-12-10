use indenter::indented;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Write};

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

impl Display for AmountDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(instructed_amount) = &self.instructed_amount {
            writeln!(f, "- Instructed amount")?;
            write!(indented(f), "{}", instructed_amount)?;
        }
        if let Some(transaction_amount) = &self.transaction_amount {
            writeln!(f, "- Transaction amount")?;
            write!(indented(f), "{}", transaction_amount)?;
        }
        if let Some(proprietary_amount) = &self.proprietary_amount {
            for proprietary_amount in proprietary_amount {
                writeln!(f, "- Proprietary amount")?;
                write!(indented(f), "{}", proprietary_amount)?;
            }
        }
        Ok(())
    }
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

impl Display for InstructedAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- Amount: {}", self.amount)
    }
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

impl Display for TransactionAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- Amount: {}", self.amount)
    }
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

impl Display for ProprietaryAmount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- Amount type: {}", self.amount_type)?;
        writeln!(f, "- Amount: {}", self.amount)?;
        Ok(())
    }
}

impl ProprietaryAmount {
    pub(crate) fn new(amount_type: String, amount: Amount) -> Self {
        Self {
            amount_type,
            amount,
        }
    }
}
