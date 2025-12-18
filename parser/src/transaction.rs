use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::collections::HashSet;
use std::fmt::Display;

/// A simple normalized representation of a bank transaction.
///
/// This type is used as a common format produced by different statement
/// parsers (e.g. MT940, CAMT.053). It contains the essential attributes
/// needed to compare or further process transactions.
#[derive(Debug, PartialEq, Hash, Eq)]
pub struct Transaction {
    amount: Decimal,
    currency: String,
    date: NaiveDate,
    account_owner: String,
    transaction_type: TransactionType,
}

impl Transaction {
    /// Creates a new `Transaction` instance.
    ///
    /// Parameters:
    /// - `amount`: transaction amount as a decimal number.
    /// - `currency`: ISO 4217 currency code (for example, `"EUR"`).
    /// - `date`: value date of the transaction.
    /// - `account_owner`: name or identifier of the account owner.
    /// - `transaction_type`: debit or credit indicator.
    ///
    /// Returns a new `Transaction`.
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

/// Indicates whether a transaction is a debit or a credit.
#[derive(Debug, PartialEq, Hash, Eq)]
pub enum TransactionType {
    /// Money is debited from the account (outgoing payment).
    Debit,
    /// Money is credited to the account (incoming payment).
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

/// A type that can provide a collection of normalized `Transaction`s.
///
/// Implement this trait for parsed statement structures to expose their
/// transactions in a unified format consumable by other parts of the
/// application (e.g. converters or comparers).
pub trait TransactionProvider {
    /// Collects transactions contained in the type.
    ///
    /// Returns a set of `Transaction`s. Implementors may perform
    /// normalization or deduplication; therefore the order is not preserved.
    fn get_transactions(&self) -> HashSet<Transaction>;
}
