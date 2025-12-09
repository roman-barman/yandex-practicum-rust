use crate::camt_053_message::statement::amount::*;
use crate::camt_053_message::statement::credit_debit_identification::*;
use crate::camt_053_message::statement::date::*;
use crate::camt_053_message::statement::entry::account_servicer_reference::*;
use crate::camt_053_message::statement::entry::additional_information_indicator::AdditionalInformationIndicator;
use crate::camt_053_message::statement::entry::bank_transaction_code::*;
use crate::camt_053_message::statement::entry::entry_details::EntryDetails;
use crate::camt_053_message::statement::entry::entry_reference::*;
use crate::camt_053_message::statement::entry::status::*;
use serde::{Deserialize, Serialize};

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
    status: Status,
    #[serde(rename = "BookgDt")]
    booking_date: Option<Date>,
    #[serde(rename = "ValDt")]
    value_date: Option<Date>,
    #[serde(rename = "AcctSvcrRef")]
    account_servicer_reference: Option<AccountServicerReference>,
    #[serde(rename = "BkTxCd")]
    bank_transaction_code: BankTransactionCode,
    #[serde(rename = "AddtlInfInd")]
    additional_information_indicator: Option<AdditionalInformationIndicator>,
    #[serde(rename = "NtryDtls")]
    entry_details: Option<Vec<EntryDetails>>,
}

impl Entry {
    pub(crate) fn new(
        reference: Option<EntryReference>,
        amount: Amount,
        credit_debit_identification: CreditDebitIdentification,
        status: Status,
        booking_date: Option<Date>,
        value_date: Option<Date>,
        account_servicer_reference: Option<AccountServicerReference>,
        bank_transaction_code: BankTransactionCode,
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
