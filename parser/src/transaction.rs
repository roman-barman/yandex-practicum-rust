use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Transaction {
    amount: Decimal,
    currency: String,
    date: NaiveDate,
    account_owner: String,
    transaction_type: TransactionType,
}

impl Transaction {
    pub fn new(
        amount: Decimal,
        currency: String,
        date: NaiveDate,
        account_owner: String,
        transaction_type: TransactionType,
    ) -> Self {
        Self {
            amount,
            currency,
            date,
            account_owner,
            transaction_type,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TransactionType {
    Debit,
    Credit,
}

impl Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionType::Debit => write!(f, "Debit"),
            TransactionType::Credit => write!(f, "Credit"),
        }
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Amount: {}", self.amount)?;
        writeln!(f, "Currency: {}", self.currency)?;
        writeln!(f, "Date: {}", self.date)?;
        writeln!(f, "Account owner: {}", self.account_owner)?;
        writeln!(f, "Transaction type: {}", self.transaction_type)?;
        Ok(())
    }
}

pub trait TransactionProvider {
    fn get_transactions(&self) -> Vec<Transaction>;
}
