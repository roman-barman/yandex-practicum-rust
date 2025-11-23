mod account_identification;
mod amount;
mod balance;
mod date;
mod related_reference;
mod statement_line;
mod statement_sequence_number;
mod transaction_reference_number;

use crate::mt_940_customer_statement_message::account_identification::*;
use crate::mt_940_customer_statement_message::balance::*;
use crate::mt_940_customer_statement_message::related_reference::*;
use crate::mt_940_customer_statement_message::statement_line::*;
use crate::mt_940_customer_statement_message::statement_sequence_number::*;
use crate::mt_940_customer_statement_message::transaction_reference_number::*;

struct Mt940CustomerStatementMessage {
    transaction_reference_number: TransactionReferenceNumber,
    related_reference: Option<RelatedReference>,
    account_identification: AccountIdentification,
    statement_sequence_no: StatementSequenceNumber,
    opening_balance: Balance,
    statement_lines: Option<Vec<StatementLine>>,
    closing_balance: Balance,
    closing_available_balance: Option<Balance>,
    forward_available_balance: Option<Balance>,
    information_to_account_owner: Option<Vec<String>>,
}
