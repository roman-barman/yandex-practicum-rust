pub(super) mod account;
pub(super) mod account_identification;
pub(super) mod amount;
pub(crate) mod balance;
pub(crate) mod credit_debit_identification;
pub(crate) mod currency;
pub(crate) mod date;
pub(crate) mod entry;
pub(super) mod financial_institution_identification;
pub(super) mod from_to_date;
pub(super) mod name;
pub(super) mod postal_address;
pub(super) mod sequence_number;
pub(super) mod transactions_summary;

use crate::Mt940CustomerStatementMessage;
use crate::camt_053_message::creation_date_time::*;
use crate::camt_053_message::identification::*;
use crate::camt_053_message::statement::account::*;
use crate::camt_053_message::statement::balance::*;
use crate::camt_053_message::statement::entry::*;
use crate::camt_053_message::statement::from_to_date::*;
use crate::camt_053_message::statement::sequence_number::*;
use crate::camt_053_message::statement::transactions_summary::*;
use indenter::indented;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Write};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Statement {
    #[serde(rename = "Id")]
    identification: Identification,
    #[serde(rename = "ElctrncSeqNb")]
    electronic_sequence_number: SequenceNumber,
    #[serde(rename = "LglSeqNb")]
    legal_sequence_number: Option<SequenceNumber>,
    #[serde(rename = "CreDtTm")]
    creation_date_time: CreationDateTime,
    #[serde(rename = "FrToDt")]
    from_to_date: Option<FromToDate>,
    #[serde(rename = "Acct")]
    account: Account,
    #[serde(rename = "Bal")]
    balances: Vec<Balance>,
    #[serde(rename = "TxsSummry")]
    transactions_summary: Option<TransactionsSummary>,
    #[serde(rename = "Ntry")]
    entries: Option<Vec<Entry>>,
}

impl From<&Mt940CustomerStatementMessage> for Statement {
    fn from(value: &Mt940CustomerStatementMessage) -> Self {
        let identification = Identification::new(
            value
                .get_transaction_reference_number()
                .as_ref()
                .to_string(),
        );
        let electronic_sequence_number = SequenceNumber::new(
            value.get_statement_sequence_number().get_statement_number() as usize,
        );
        let legal_sequence_number = value
            .get_statement_sequence_number()
            .get_sequence_number()
            .map(|val| SequenceNumber::new(val as usize));
        let creation_date_time = CreationDateTime::now();
        let account = Account::from(value);
        let balances = vec![
            Balance::from_opening_balance(value.get_opening_balance()),
            Balance::from_closing_balance(value.get_closing_balance()),
        ];
        let entries = Entry::from_mt_940(value);
        Self {
            identification,
            electronic_sequence_number,
            legal_sequence_number,
            creation_date_time,
            from_to_date: None,
            account,
            balances,
            transactions_summary: None,
            entries,
        }
    }
}

impl Statement {
    pub(crate) fn get_identification(&self) -> &str {
        self.identification.as_ref()
    }

    pub(crate) fn get_account_identification(&self) -> &str {
        self.account.get_identification()
    }

    pub(crate) fn get_electronic_sequence_number(&self) -> &usize {
        self.electronic_sequence_number.as_ref()
    }

    pub(crate) fn get_legal_sequence_number(&self) -> Option<&usize> {
        self.legal_sequence_number.as_ref().map(|s| s.as_ref())
    }

    pub(crate) fn get_opening_balance(&self) -> Option<&Balance> {
        self.balances
            .iter()
            .find(|balance| balance.is_opening_balance())
    }

    pub(crate) fn get_closing_balance(&self) -> Option<&Balance> {
        self.balances
            .iter()
            .find(|balance| balance.is_closing_balance())
    }

    pub(crate) fn get_entries(&self) -> Option<&Vec<Entry>> {
        self.entries.as_ref()
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- Identification: {}", self.identification)?;
        writeln!(
            f,
            "- Electronic sequence number: {}",
            self.electronic_sequence_number
        )?;
        if let Some(legal_sequence_number) = &self.legal_sequence_number {
            writeln!(f, "- Legal sequence number: {}", legal_sequence_number)?;
        }
        writeln!(f, "- Creation date time: {}", self.creation_date_time)?;
        if let Some(from_to_date) = &self.from_to_date {
            writeln!(f, "- From to date: {}", from_to_date)?;
        }
        writeln!(f, "- Account")?;
        write!(indented(f), "{}", self.account)?;
        for balance in &self.balances {
            writeln!(f, "- Balance")?;
            write!(indented(f), "{}", balance)?;
        }
        if let Some(transactions_summary) = &self.transactions_summary {
            writeln!(f, "- Transactions summary")?;
            write!(indented(f), "{}", transactions_summary)?;
        }
        if let Some(entries) = &self.entries {
            for entry in entries {
                writeln!(f, "- Entry")?;
                write!(indented(f), "{}", entry)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
impl Statement {
    pub(super) fn new(
        identification: Identification,
        electronic_sequence_number: SequenceNumber,
        legal_sequence_number: Option<SequenceNumber>,
        creation_date_time: CreationDateTime,
        from_to_date: Option<FromToDate>,
        account: Account,
        balances: Vec<Balance>,
        transactions_summary: Option<TransactionsSummary>,
        entries: Option<Vec<Entry>>,
    ) -> Self {
        Self {
            identification,
            electronic_sequence_number,
            legal_sequence_number,
            creation_date_time,
            from_to_date,
            account,
            balances,
            transactions_summary,
            entries,
        }
    }
}
