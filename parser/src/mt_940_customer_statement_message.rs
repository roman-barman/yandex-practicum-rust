mod account_identification;
mod related_reference;
mod transaction_reference_number;

use crate::mt_940_customer_statement_message::account_identification::*;
use crate::mt_940_customer_statement_message::related_reference::*;
use crate::mt_940_customer_statement_message::transaction_reference_number::*;
use chrono::NaiveDate;
use rust_decimal::Decimal;

struct Mt940CustomerStatementMessage {
    transaction_reference_number: TransactionReferenceNumber,
    related_reference: Option<RelatedReference>,
    account_identification: AccountIdentification,
    statement_no: String,
    sequence_number: String,
    opening_balance: OpeningBalance,
    statement_lines: Option<Vec<StatementLine>>,
    closing_balance: ClosingBalance,
    closing_available_balance: Option<ClosingAvailableBalance>,
    forward_available_balance: Option<ForwardAvailableBalance>,
    information_to_account_owner: Option<Vec<String>>,
}

struct OpeningBalance {
    is_intermediate: bool,
    debit_credit_mark: CreditDebitMark,
    date: NaiveDate,
    currency_code: String,
    amount: Decimal,
}

enum CreditDebitMark {
    Credit,
    Debit,
}

struct StatementLine {
    value_date: NaiveDate,
    entry_date: Option<NaiveDate>,
    debit_credit_mark: StatementLineMark,
    funds_code: Option<char>,
    amount: Decimal,
    transaction_type_identification_code: String,
    account_owner_ref: String,
    bank_ref: Option<String>,
    supplementary_details: Option<String>,
    information_to_account_owner: Option<Vec<String>>,
}

enum StatementLineMark {
    Credit,
    Debit,
    ReversalOfCredit,
    ReversalOfDebit,
}

struct ClosingBalance {
    is_intermediate: bool,
    debit_credit_mark: CreditDebitMark,
    date: NaiveDate,
    currency_code: String,
    amount: Decimal,
}

struct ClosingAvailableBalance {
    debit_credit_mark: CreditDebitMark,
    date: NaiveDate,
    currency_code: String,
    amount: Decimal,
}

struct ForwardAvailableBalance {
    debit_credit_mark: CreditDebitMark,
    date: NaiveDate,
    currency_code: String,
    amount: Decimal,
}
