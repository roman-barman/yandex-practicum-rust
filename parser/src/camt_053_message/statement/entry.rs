use crate::Mt940CustomerStatementMessage;
use crate::camt_053_message::statement::amount::*;
use crate::camt_053_message::statement::credit_debit_identification::*;
use crate::camt_053_message::statement::currency::Currency;
use crate::camt_053_message::statement::date::*;
use crate::camt_053_message::statement::entry::account_servicer_reference::*;
use crate::camt_053_message::statement::entry::additional_information_indicator::AdditionalInformationIndicator;
use crate::camt_053_message::statement::entry::bank_transaction_code::*;
use crate::camt_053_message::statement::entry::entry_details::EntryDetails;
use crate::camt_053_message::statement::entry::entry_reference::*;
use crate::camt_053_message::statement::entry::status::*;
use indenter::indented;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Write};

pub(crate) mod account_servicer_reference;
pub(crate) mod additional_information_indicator;
pub(crate) mod bank_transaction_code;
pub(crate) mod entry_details;
pub(crate) mod entry_reference;
pub(crate) mod status;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct Entry {
    #[serde(rename = "NtryRef")]
    reference: Option<EntryReference>,
    #[serde(rename = "Amt")]
    amount: Amount,
    #[serde(rename = "CdtDbtInd")]
    credit_debit_identification: CreditDebitIdentification,
    #[serde(rename = "Sts")]
    status: Option<Status>,
    #[serde(rename = "BookgDt")]
    booking_date: Option<Date>,
    #[serde(rename = "ValDt")]
    value_date: Option<Date>,
    #[serde(rename = "AcctSvcrRef")]
    account_servicer_reference: Option<AccountServicerReference>,
    #[serde(rename = "BkTxCd")]
    bank_transaction_code: Option<BankTransactionCode>,
    #[serde(rename = "AddtlInfInd")]
    additional_information_indicator: Option<AdditionalInformationIndicator>,
    #[serde(rename = "NtryDtls")]
    entry_details: Option<Vec<EntryDetails>>,
}

impl Entry {
    pub(crate) fn get_entry_details(&self) -> Option<&Vec<EntryDetails>> {
        self.entry_details.as_ref()
    }

    pub(crate) fn get_value_date(&self) -> Option<&Date> {
        self.value_date.as_ref()
    }

    pub(crate) fn get_booking_date(&self) -> Option<&Date> {
        self.booking_date.as_ref()
    }

    pub(crate) fn get_credit_debit_identification(&self) -> &CreditDebitIdentification {
        &self.credit_debit_identification
    }

    pub(crate) fn get_amount(&self) -> &Amount {
        &self.amount
    }

    pub(crate) fn get_account_servicer_reference(&self) -> Option<&AccountServicerReference> {
        self.account_servicer_reference.as_ref()
    }

    pub(crate) fn get_bank_transaction_code(&self) -> Option<&BankTransactionCode> {
        self.bank_transaction_code.as_ref()
    }

    pub(super) fn from_mt_940(value: &Mt940CustomerStatementMessage) -> Option<Vec<Entry>> {
        if value.get_statement_lines().is_none() {
            return None;
        }

        let mut result = vec![];
        let currency = value.get_opening_balance().get_currency_code().as_ref();

        for statement in value.get_statement_lines().unwrap() {
            let value_date = Some(Date::from(statement.get_value_date()));
            let booking_date = statement.get_entry_date().map(Date::from);
            let credit_debit_identification =
                CreditDebitIdentification::from(statement.get_debit_credit_mark());
            let amount = Amount::new(
                Currency::new(currency.to_string()),
                statement.get_amount().as_ref().clone(),
            );
            let account_servicer_reference = Some(AccountServicerReference::from(
                statement.get_account_owner_ref(),
            ));
            result.push(Entry {
                reference: None,
                amount,
                credit_debit_identification,
                status: None,
                booking_date,
                value_date,
                account_servicer_reference,
                bank_transaction_code: None,
                additional_information_indicator: None,
                entry_details: None,
            })
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

impl Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(reference) = &self.reference {
            writeln!(f, "- Reference: {}", reference)?;
        }
        writeln!(f, "- Amount: {}", self.amount)?;
        writeln!(
            f,
            "- Credit/debit identification: {}",
            self.credit_debit_identification
        )?;
        if let Some(status) = &self.status {
            writeln!(f, "- Status: {}", status)?;
        }
        if let Some(booking_date) = &self.booking_date {
            writeln!(f, "- Booking date: {}", booking_date)?;
        }
        if let Some(value_date) = &self.value_date {
            writeln!(f, "- Value date: {}", value_date)?;
        }
        if let Some(account_servicer_reference) = &self.account_servicer_reference {
            writeln!(
                f,
                "- Account servicer reference: {}",
                account_servicer_reference
            )?;
        }
        if let Some(bank_transaction_code) = &self.bank_transaction_code {
            writeln!(f, "- Bank transaction code")?;
            write!(indented(f), "{}", bank_transaction_code)?;
        }
        if let Some(additional_information_indicator) = &self.additional_information_indicator {
            writeln!(f, "- Additional information indicator")?;
            write!(indented(f), "{}", additional_information_indicator)?;
        }
        if let Some(entry_details) = &self.entry_details {
            for entry_detail in entry_details {
                writeln!(f, "- Entry detail")?;
                write!(indented(f), "{}", entry_detail)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
impl Entry {
    pub(crate) fn new(
        reference: Option<EntryReference>,
        amount: Amount,
        credit_debit_identification: CreditDebitIdentification,
        status: Option<Status>,
        booking_date: Option<Date>,
        value_date: Option<Date>,
        account_servicer_reference: Option<AccountServicerReference>,
        bank_transaction_code: Option<BankTransactionCode>,
        additional_information_indicator: Option<AdditionalInformationIndicator>,
        entry_details: Option<Vec<EntryDetails>>,
    ) -> Self {
        Self {
            reference,
            amount,
            credit_debit_identification,
            status,
            booking_date,
            value_date,
            account_servicer_reference,
            bank_transaction_code,
            additional_information_indicator,
            entry_details,
        }
    }
}
