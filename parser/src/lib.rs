#![deny(unreachable_pub)]
#![warn(missing_docs)]

//! Parser and converter for bank statement formats (MT940 and CAMT.053).
//!
//! This crate provides:
//! - Parsers for SWIFT MT940 and ISO 20022 CAMT.053 statements
//! - Converters between supported formats
//! - A normalized `Transaction` type and a `TransactionProvider` trait
//!
//! Public types are re-exported at the crate root for convenience.
mod camt_053_message;
mod message_writer;
mod mt_940_customer_statement_message;
mod transaction;

pub use camt_053_message::Camt053Message;
pub use camt_053_message::error::Camt053MessageError;
pub use camt_053_message::statement::Statement as Camt053Statement;
pub use message_writer::MessageWriter;
pub use mt_940_customer_statement_message::Mt940CustomerStatementMessage;
pub use mt_940_customer_statement_message::error::Mt940CustomerStatementMessageReadError;
pub use transaction::{Transaction, TransactionProvider, TransactionType};
