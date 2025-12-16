use crate::Transaction;
use crate::camt_053_message::statement::entry::Entry;
use crate::mt_940_customer_statement_message::amount::*;
use crate::mt_940_customer_statement_message::date::*;
use crate::mt_940_customer_statement_message::information_to_account_owner::*;
use crate::mt_940_customer_statement_message::statement_line::account_owner_ref::*;
use crate::mt_940_customer_statement_message::statement_line::bank_ref::*;
use crate::mt_940_customer_statement_message::statement_line::credit_debit_mark::*;
use crate::mt_940_customer_statement_message::statement_line::funds_code::*;
use crate::mt_940_customer_statement_message::statement_line::identification_code::*;
use crate::mt_940_customer_statement_message::statement_line::supplementary_details::*;
use crate::mt_940_customer_statement_message::statement_line::transaction_type::*;
use chrono::NaiveDate;
use std::any::Any;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;
use std::num::ParseIntError;
use std::ops::Add;

pub(crate) mod account_owner_ref;
mod bank_ref;
pub(crate) mod credit_debit_mark;
mod funds_code;
mod identification_code;
pub(crate) mod supplementary_details;
mod transaction_type;
const ENTRY_DATE_LENGTH: usize = 4;

#[derive(Debug, PartialEq)]
pub(crate) struct StatementLine {
    value_date: Date,
    entry_date: Option<Date>,
    debit_credit_mark: CreditDebitMark,
    funds_code: Option<FundsCode>,
    amount: Amount,
    transaction_type: TransactionType,
    identification_code: IdentificationCode,
    account_owner_ref: AccountOwnerRef,
    bank_ref: Option<BankRef>,
    supplementary_details: Option<SupplementaryDetails>,
    information_to_account_owner: Option<InformationToAccountOwner>,
}

impl StatementLine {
    pub(super) fn try_from_entry(
        value: &Entry,
    ) -> Result<Vec<StatementLine>, StatementLineParseError> {
        let mut result = Vec::new();
        let value_date = Date::from(value.get_value_date().ok_or(DateParseError::Empty)?);
        let entry_date = value.get_booking_date().map(Date::from);
        let debit_credit_mark = CreditDebitMark::try_from(value.get_credit_debit_identification())?;
        let transaction_type = TransactionType::NonSwiftTransfer;
        let identification_code = IdentificationCode::try_from("TRF")?;
        let account_owner_ref = AccountOwnerRef::try_from(
            value
                .get_account_servicer_reference()
                .map_or("NOREF", |account_owner_ref| account_owner_ref.as_ref()),
        )?;
        let information_to_account_owner_from_code = value
            .get_bank_transaction_code()
            .and_then(InformationToAccountOwner::from_bank_transaction_code);

        for entry_details in value
            .get_entry_details()
            .ok_or(StatementLineParseError::Empty)?
        {
            for transaction in entry_details
                .get_transaction_details()
                .ok_or(StatementLineParseError::Empty)?
            {
                let amount = transaction
                    .get_amount_details()
                    .and_then(|details| {
                        details
                            .get_transaction_amount()
                            .map(|amount| Amount::new(amount.get_amount().get_amount()))
                    })
                    .unwrap_or(Amount::new(value.get_amount().get_amount()));
                let additional_information =
                    InformationToAccountOwner::from_transaction_details(transaction);
                let information_to_account_owner_from = if let Some(ref info_from_code) =
                    information_to_account_owner_from_code
                    && let Some(additional_info) = additional_information
                {
                    Some((*info_from_code).clone().add(additional_info))
                } else if let Some(ref info_from_code) = information_to_account_owner_from_code {
                    Some((*info_from_code).clone())
                } else {
                    additional_information
                };

                result.push(Self {
                    value_date: value_date.clone(),
                    entry_date: entry_date.clone(),
                    debit_credit_mark: debit_credit_mark.clone(),
                    funds_code: None,
                    amount,
                    transaction_type: transaction_type.clone(),
                    identification_code: identification_code.clone(),
                    account_owner_ref: account_owner_ref.clone(),
                    bank_ref: None,
                    supplementary_details: None,
                    information_to_account_owner: information_to_account_owner_from,
                })
            }
        }

        if result.is_empty() {
            Err(StatementLineParseError::Empty)
        } else {
            Ok(result)
        }
    }

    pub(crate) fn to_transaction(&self, currency: String) -> Transaction {
        let transaction_type = match self.debit_credit_mark {
            CreditDebitMark::Debit => crate::transaction::TransactionType::Debit,
            CreditDebitMark::Credit => crate::transaction::TransactionType::Credit,
            CreditDebitMark::ReversalOfCredit => crate::transaction::TransactionType::Debit,
            CreditDebitMark::ReversalOfDebit => crate::transaction::TransactionType::Credit,
        };
        Transaction::new(
            *self.amount.as_ref(),
            currency,
            *self.value_date.as_ref(),
            self.account_owner_ref.to_string(),
            transaction_type,
        )
    }

    pub(crate) fn get_value_date(&self) -> &Date {
        &self.value_date
    }

    pub(crate) fn get_entry_date(&self) -> Option<&Date> {
        self.entry_date.as_ref()
    }

    pub(crate) fn get_debit_credit_mark(&self) -> &CreditDebitMark {
        &self.debit_credit_mark
    }

    pub(crate) fn get_amount(&self) -> &Amount {
        &self.amount
    }

    pub(crate) fn get_account_owner_ref(&self) -> &AccountOwnerRef {
        &self.account_owner_ref
    }

    pub(super) fn add_supplementary_details(
        &mut self,
        value: SupplementaryDetails,
    ) -> Result<(), StatementLineError> {
        if self.supplementary_details.is_some() {
            return Err(StatementLineError::SupplementaryDetailsAlreadySet);
        }
        self.supplementary_details = Some(value);
        Ok(())
    }

    pub(super) fn add_information_to_account_owner(
        &mut self,
        value: InformationToAccountOwner,
    ) -> Result<(), StatementLineError> {
        match self.information_to_account_owner {
            Some(ref mut information_to_account_owner) => {
                information_to_account_owner.try_add(value)?;
            }
            None => {
                self.information_to_account_owner = Some(value);
            }
        }
        Ok(())
    }

    pub(super) fn get_information_to_account_owner(&self) -> Option<&InformationToAccountOwner> {
        self.information_to_account_owner.as_ref()
    }

    pub(super) fn statement_line_write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.value_date.write_to(writer)?;
        if let Some(entry_date) = &self.entry_date {
            entry_date.write_without_year_to(writer)?;
        }
        self.debit_credit_mark.write_to(writer)?;
        if let Some(funds_code) = &self.funds_code {
            funds_code.write_to(writer)?;
        }
        self.amount.write_to(writer)?;
        self.transaction_type.write_to(writer)?;
        self.identification_code.write_to(writer)?;
        self.account_owner_ref.write_to(writer)?;
        if let Some(bank_ref) = &self.bank_ref {
            writer.write_all(b"//")?;
            bank_ref.write_to(writer)?;
        }
        if let Some(supplementary_details) = &self.supplementary_details {
            writer.write_all(b"\r\n")?;
            supplementary_details.write_to(writer)?;
        }
        Ok(())
    }
}

impl Display for StatementLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "- Value date: {}", self.value_date)?;
        if let Some(entry_date) = &self.entry_date {
            writeln!(f, "- Entry date: {}", entry_date)?;
        }
        writeln!(f, "- Debit/Credit: {}", self.debit_credit_mark)?;
        if let Some(funds_code) = &self.funds_code {
            writeln!(f, "- Funds code: {}", funds_code)?;
        }
        writeln!(f, "- Amount: {}", self.amount)?;
        writeln!(f, "- Transaction type: {}", self.transaction_type)?;
        writeln!(f, "- Identification code: {}", self.identification_code)?;
        writeln!(f, "- Account owner reference: {}", self.account_owner_ref)?;
        if let Some(bank_ref) = &self.bank_ref {
            writeln!(f, "- Bank reference: {}", bank_ref)?;
        }
        if let Some(supplementary_details) = &self.supplementary_details {
            writeln!(f, "- Supplementary details: {}", supplementary_details)?;
        }
        if let Some(information_to_account_owner) = &self.information_to_account_owner {
            writeln!(
                f,
                "- Information to account owner: {}",
                information_to_account_owner
            )
        } else {
            Ok(())
        }
    }
}

impl TryFrom<&str> for StatementLine {
    type Error = StatementLineParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(StatementLineParseError::Empty);
        }
        let mut cursor = 0;
        let value_date = read_value_date(value, &mut cursor)?;
        let entry_date = read_entry_date(value, &value_date, &mut cursor)?;
        let debit_credit_mark = read_debit_credit_mark(value, &mut cursor)?;
        let funds_code = read_funds_code(value, &mut cursor)?;
        let amount = read_amount(value, &mut cursor)?;
        let transaction_type = read_transaction_type(value, &mut cursor)?;
        let identification_code = read_identification_code(value, &mut cursor)?;
        let account_owner_ref = read_account_owner_ref(value, &mut cursor)?;
        let bank_ref = read_bank_ref(value, &mut cursor)?;

        Ok(Self {
            value_date,
            entry_date,
            debit_credit_mark,
            funds_code,
            amount,
            transaction_type,
            identification_code,
            account_owner_ref,
            bank_ref,
            supplementary_details: None,
            information_to_account_owner: None,
        })
    }
}

fn read_value_date(line: &str, cursor: &mut usize) -> Result<Date, StatementLineParseError> {
    let value_date_str: String = line.chars().take(DATE_LENGTH).collect();
    *cursor += DATE_LENGTH;
    Ok(Date::try_from(value_date_str.as_str())?)
}

fn read_entry_date(
    line: &str,
    value_date: &Date,
    cursor: &mut usize,
) -> Result<Option<Date>, StatementLineParseError> {
    let entry_date: String = line
        .chars()
        .skip(*cursor)
        .take_while(|c| c.is_ascii_digit())
        .collect();
    if entry_date.is_empty() {
        return Ok(None);
    }
    if entry_date.len() != ENTRY_DATE_LENGTH {
        return Err(StatementLineParseError::InvalidFormat(None));
    }

    let month = entry_date
        .chars()
        .take(2)
        .collect::<String>()
        .parse::<u32>()?;
    let day = entry_date
        .chars()
        .skip(2)
        .take(2)
        .collect::<String>()
        .parse::<u32>()?;
    let (year, _, _) = value_date.ymd_date();
    let date = NaiveDate::from_ymd_opt(year, month, day)
        .ok_or(StatementLineParseError::InvalidFormat(None))?;
    *cursor += ENTRY_DATE_LENGTH;
    Ok(Some(Date::new(date)))
}

fn read_debit_credit_mark(
    line: &str,
    cursor: &mut usize,
) -> Result<CreditDebitMark, StatementLineParseError> {
    let value = line
        .chars()
        .skip(*cursor)
        .take_while(|c| c.is_ascii_alphabetic())
        .collect::<String>();
    if value.is_empty() {
        return Err(StatementLineParseError::InvalidFormat(None));
    }

    if value.len() > CREDIT_DEBIT_MARK_MAX_LENGTH + 1 {
        return Err(StatementLineParseError::InvalidFormat(None));
    }

    let mut len = CREDIT_DEBIT_MARK_MAX_LENGTH;
    while len > 1 {
        let value = line.chars().skip(*cursor).take(len).collect::<String>();
        let credit_debit_mark_parse_result = CreditDebitMark::try_from(value.as_str());
        match credit_debit_mark_parse_result {
            Ok(credit_debit_mark) => {
                *cursor += len;
                return Ok(credit_debit_mark);
            }
            Err(_) => {
                len -= 1;
            }
        }
    }

    let value = line.chars().skip(*cursor).take(1).collect::<String>();
    *cursor += 1;
    Ok(CreditDebitMark::try_from(value.as_str())?)
}

fn read_funds_code(
    line: &str,
    cursor: &mut usize,
) -> Result<Option<FundsCode>, StatementLineParseError> {
    let funds_code = line.chars().nth(*cursor);
    match funds_code {
        None => Ok(None),
        Some(funds_code) => {
            if funds_code.is_ascii_alphabetic() {
                *cursor += 1;
                Ok(Some(FundsCode::try_from(&funds_code)?))
            } else {
                Ok(None)
            }
        }
    }
}

fn read_amount(line: &str, cursor: &mut usize) -> Result<Amount, StatementLineParseError> {
    let amount = line
        .chars()
        .skip(*cursor)
        .take_while(|c| c.is_ascii_digit() || c == &',')
        .collect::<String>();
    *cursor += amount.len();
    Ok(Amount::try_from(amount.as_str())?)
}

fn read_transaction_type(
    line: &str,
    cursor: &mut usize,
) -> Result<TransactionType, StatementLineParseError> {
    let transaction_type = line.chars().nth(*cursor);
    match transaction_type {
        None => Err(StatementLineParseError::InvalidFormat(None)),
        Some(transaction_type) => {
            *cursor += 1;
            Ok(TransactionType::try_from(&transaction_type)?)
        }
    }
}

fn read_identification_code(
    line: &str,
    cursor: &mut usize,
) -> Result<IdentificationCode, StatementLineParseError> {
    let identification_code = line
        .chars()
        .skip(*cursor)
        .take(IDENTIFICATION_CODE_LENGTH)
        .collect::<String>();
    *cursor += IDENTIFICATION_CODE_LENGTH;
    Ok(IdentificationCode::try_from(identification_code.as_str())?)
}

fn read_account_owner_ref(
    line: &str,
    cursor: &mut usize,
) -> Result<AccountOwnerRef, StatementLineParseError> {
    let mut read_len = 0;
    let chars = line.chars().skip(*cursor);
    let mut value = String::new();
    for c in chars {
        if c == '/' && value.ends_with('/') {
            value.pop();
            read_len -= 1;
            break;
        }
        read_len += 1;
        value.push(c);
    }

    *cursor += read_len;
    Ok(AccountOwnerRef::try_from(value.as_str())?)
}

fn read_bank_ref(
    line: &str,
    cursor: &mut usize,
) -> Result<Option<BankRef>, StatementLineParseError> {
    let bank_ref = line.chars().skip(*cursor).collect::<String>();
    if bank_ref.is_empty() {
        return Ok(None);
    }
    if bank_ref.starts_with("//") {
        *cursor += bank_ref.len();
        Ok(Some(BankRef::try_from(
            bank_ref.strip_prefix("//").unwrap_or(""),
        )?))
    } else {
        Err(StatementLineParseError::InvalidFormat(None))
    }
}

#[derive(Debug)]
pub(crate) enum StatementLineParseError {
    Empty,
    InvalidFormat(Option<Box<dyn Error>>),
}

impl PartialEq for StatementLineParseError {
    fn eq(&self, other: &Self) -> bool {
        match self {
            StatementLineParseError::Empty => matches!(other, StatementLineParseError::Empty),
            StatementLineParseError::InvalidFormat(None) => {
                matches!(other, StatementLineParseError::InvalidFormat(None))
            }
            StatementLineParseError::InvalidFormat(Some(err1)) => {
                if let StatementLineParseError::InvalidFormat(Some(err2)) = other {
                    (*err1).type_id() == (*err2).type_id()
                } else {
                    false
                }
            }
        }
    }
}

impl Display for StatementLineParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StatementLineParseError::Empty => write!(f, "Statement line is empty"),
            StatementLineParseError::InvalidFormat(None) => {
                write!(f, "Statement line has invalid format")
            }
            StatementLineParseError::InvalidFormat(Some(err)) => {
                write!(f, "Statement line has invalid format: {}", err)
            }
        }
    }
}

impl From<DateParseError> for StatementLineParseError {
    fn from(value: DateParseError) -> Self {
        Self::InvalidFormat(Some(Box::new(value)))
    }
}

impl From<CreditDebitMarkParseError> for StatementLineParseError {
    fn from(value: CreditDebitMarkParseError) -> Self {
        Self::InvalidFormat(Some(Box::new(value)))
    }
}

impl From<TransactionTypeParseError> for StatementLineParseError {
    fn from(value: TransactionTypeParseError) -> Self {
        Self::InvalidFormat(Some(Box::new(value)))
    }
}

impl From<AmountParseError> for StatementLineParseError {
    fn from(value: AmountParseError) -> Self {
        Self::InvalidFormat(Some(Box::new(value)))
    }
}

impl From<FundsCodeParseError> for StatementLineParseError {
    fn from(value: FundsCodeParseError) -> Self {
        Self::InvalidFormat(Some(Box::new(value)))
    }
}

impl From<AccountOwnerRefParseError> for StatementLineParseError {
    fn from(value: AccountOwnerRefParseError) -> Self {
        Self::InvalidFormat(Some(Box::new(value)))
    }
}

impl From<BankRefParseError> for StatementLineParseError {
    fn from(value: BankRefParseError) -> Self {
        Self::InvalidFormat(Some(Box::new(value)))
    }
}

impl From<IdentificationCodeParseError> for StatementLineParseError {
    fn from(value: IdentificationCodeParseError) -> Self {
        Self::InvalidFormat(Some(Box::new(value)))
    }
}

impl From<ParseIntError> for StatementLineParseError {
    fn from(value: ParseIntError) -> Self {
        Self::InvalidFormat(Some(Box::new(value)))
    }
}

impl Error for StatementLineParseError {}

#[derive(Debug, PartialEq)]
pub(super) enum StatementLineError {
    SupplementaryDetailsAlreadySet,
    InformationToAccountOwnerTooLong,
}

impl From<InformationToAccountOwnerError> for StatementLineError {
    fn from(value: InformationToAccountOwnerError) -> Self {
        match value {
            InformationToAccountOwnerError::TooLong => Self::InformationToAccountOwnerTooLong,
        }
    }
}

impl Display for StatementLineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StatementLineError::SupplementaryDetailsAlreadySet => {
                write!(f, "Supplementary details already set")
            }
            StatementLineError::InformationToAccountOwnerTooLong => {
                write!(f, "Information to account owner too long")
            }
        }
    }
}

impl Error for StatementLineError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const DATA: &str = "2303010228CK366336,2NTRFArbi/deposit//1323333800";

    #[test]
    fn test_statement_line_write_to() {
        let mut buffer = Cursor::new(Vec::new());
        StatementLine::try_from(DATA)
            .unwrap()
            .statement_line_write_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer.get_ref(), DATA.as_bytes());

        let mut buffer = Cursor::new(Vec::new());
        let mut line = StatementLine::try_from(DATA).unwrap();
        line.add_supplementary_details(SupplementaryDetails::try_from("test").unwrap())
            .unwrap();
        line.statement_line_write_to(&mut buffer).unwrap();
        assert_eq!(
            buffer.get_ref(),
            b"2303010228CK366336,2NTRFArbi/deposit//1323333800\r\ntest"
        );
    }

    #[test]
    fn test_read_value_date() {
        let mut cursor = 0;
        let result = read_value_date(DATA, &mut cursor);
        assert_eq!(
            result,
            Ok(Date::new(NaiveDate::from_ymd_opt(2023, 3, 1).unwrap()))
        );
        assert_eq!(cursor, DATE_LENGTH);
    }

    #[test]
    fn test_read_entry_date_invalid_format() {
        let mut cursor = DATE_LENGTH;
        let result = read_entry_date(
            "230301228CK366336,2NTRFArbi/deposit//1323333800",
            &Date::new(NaiveDate::from_ymd_opt(2023, 3, 1).unwrap()),
            &mut cursor,
        );
        assert_eq!(result, Err(StatementLineParseError::InvalidFormat(None)));
    }

    #[test]
    fn test_read_entry_date_none() {
        let mut cursor = DATE_LENGTH;
        let result = read_entry_date(
            "230301CK366336,2NTRFArbi/deposit//1323333800",
            &Date::new(NaiveDate::from_ymd_opt(2023, 3, 1).unwrap()),
            &mut cursor,
        );
        assert_eq!(result, Ok(None));
    }

    #[test]
    fn test_read_entry_date() {
        let mut cursor = DATE_LENGTH;
        let result = read_entry_date(
            DATA,
            &Date::new(NaiveDate::from_ymd_opt(2023, 3, 1).unwrap()),
            &mut cursor,
        );
        assert_eq!(
            result,
            Ok(Some(Date::new(
                NaiveDate::from_ymd_opt(2023, 2, 28).unwrap()
            )))
        );
        assert_eq!(cursor, DATE_LENGTH + ENTRY_DATE_LENGTH);
    }

    #[test]
    fn test_read_debit_credit_mark_invalid_format() {
        let mut cursor = DATE_LENGTH + ENTRY_DATE_LENGTH;
        let data = "2303010228366336,2NTRFArbi/deposit//1323333800";
        let result = read_debit_credit_mark(data, &mut cursor);
        assert_eq!(result, Err(StatementLineParseError::InvalidFormat(None)));

        let data = "2303010228CKPP366336,2NTRFArbi/deposit//1323333800";
        let result = read_debit_credit_mark(data, &mut cursor);
        assert_eq!(result, Err(StatementLineParseError::InvalidFormat(None)));
    }

    #[test]
    fn test_read_debit_credit_mark() {
        let mut cursor = DATE_LENGTH + ENTRY_DATE_LENGTH;
        let result = read_debit_credit_mark(DATA, &mut cursor);
        assert_eq!(result, Ok(CreditDebitMark::Credit));
        assert_eq!(cursor, DATE_LENGTH + ENTRY_DATE_LENGTH + 1);

        let mut cursor = DATE_LENGTH + ENTRY_DATE_LENGTH;
        let data = "2303010228RCK366336,2NTRFArbi/deposit//1323333800";
        let result = read_debit_credit_mark(data, &mut cursor);
        assert_eq!(result, Ok(CreditDebitMark::ReversalOfCredit));
        assert_eq!(cursor, DATE_LENGTH + ENTRY_DATE_LENGTH + 2);
    }

    #[test]
    fn test_read_funds_code() {
        let mut cursor = DATE_LENGTH + ENTRY_DATE_LENGTH + 1;
        let result = read_funds_code(DATA, &mut cursor);
        assert_eq!(result, Ok(Some(FundsCode::try_from(&'K').unwrap())));
        assert_eq!(cursor, DATE_LENGTH + ENTRY_DATE_LENGTH + 2);

        let mut cursor = DATE_LENGTH + ENTRY_DATE_LENGTH + 1;
        let data = "2303010228C366336,2NTRFArbi/deposit//1323333800";
        let result = read_funds_code(data, &mut cursor);
        assert_eq!(result, Ok(None));
        assert_eq!(cursor, DATE_LENGTH + ENTRY_DATE_LENGTH + 1);
    }

    #[test]
    fn test_read_amount() {
        let mut cursor = DATE_LENGTH + ENTRY_DATE_LENGTH + 2;
        let result = read_amount(DATA, &mut cursor);
        assert_eq!(result, Ok(Amount::try_from("366336,2").unwrap()));
        assert_eq!(cursor, DATE_LENGTH + ENTRY_DATE_LENGTH + 10);
    }

    #[test]
    fn test_read_transaction_type_invalid_format() {
        let mut cursor = DATE_LENGTH + ENTRY_DATE_LENGTH + 10;
        let data = "2303010228CK366336,2";
        let result = read_transaction_type(data, &mut cursor);
        assert_eq!(result, Err(StatementLineParseError::InvalidFormat(None)));
    }

    #[test]
    fn test_read_transaction_type() {
        let mut cursor = DATE_LENGTH + ENTRY_DATE_LENGTH + 10;
        let result = read_transaction_type(DATA, &mut cursor);
        assert_eq!(result, Ok(TransactionType::NonSwiftTransfer));
        assert_eq!(cursor, DATE_LENGTH + ENTRY_DATE_LENGTH + 11);
    }

    #[test]
    fn test_read_identification_code() {
        let mut cursor = DATE_LENGTH + ENTRY_DATE_LENGTH + 11;
        let result = read_identification_code(DATA, &mut cursor);
        assert_eq!(result, Ok(IdentificationCode::try_from("TRF").unwrap()));
        assert_eq!(
            cursor,
            DATE_LENGTH + ENTRY_DATE_LENGTH + 11 + IDENTIFICATION_CODE_LENGTH
        );
    }

    #[test]
    fn test_read_account_owner_ref() {
        let mut cursor = DATE_LENGTH + ENTRY_DATE_LENGTH + 11 + IDENTIFICATION_CODE_LENGTH;
        let result = read_account_owner_ref(DATA, &mut cursor);
        assert_eq!(
            result,
            Ok(AccountOwnerRef::try_from("Arbi/deposit").unwrap())
        );
        assert_eq!(
            cursor,
            DATE_LENGTH + ENTRY_DATE_LENGTH + 11 + IDENTIFICATION_CODE_LENGTH + 12
        );
    }

    #[test]
    fn test_read_bank_ref() {
        let mut cursor = DATE_LENGTH + ENTRY_DATE_LENGTH + 11 + IDENTIFICATION_CODE_LENGTH + 12;
        let result = read_bank_ref(DATA, &mut cursor);
        assert_eq!(result, Ok(Some(BankRef::try_from("1323333800").unwrap())));
        assert_eq!(
            cursor,
            DATE_LENGTH + ENTRY_DATE_LENGTH + 11 + IDENTIFICATION_CODE_LENGTH + 24
        );

        let mut cursor = DATE_LENGTH + ENTRY_DATE_LENGTH + 11 + IDENTIFICATION_CODE_LENGTH + 12;
        let data = "2303010228CK366336,2NTRFArbi/deposit";
        let result = read_bank_ref(data, &mut cursor);
        assert_eq!(result, Ok(None));
        assert_eq!(
            cursor,
            DATE_LENGTH + ENTRY_DATE_LENGTH + 11 + IDENTIFICATION_CODE_LENGTH + 12
        );
    }

    #[test]
    fn test_empty_statement_line() {
        let result = StatementLine::try_from("");
        assert_eq!(result, Err(StatementLineParseError::Empty));
        assert_eq!(result.unwrap_err().to_string(), "Statement line is empty");
    }

    #[test]
    fn test_statement_line() {
        let result = StatementLine::try_from(DATA);
        assert_eq!(
            result,
            Ok(StatementLine {
                value_date: Date::new(NaiveDate::from_ymd_opt(2023, 3, 1).unwrap()),
                entry_date: Some(Date::new(NaiveDate::from_ymd_opt(2023, 2, 28).unwrap())),
                debit_credit_mark: CreditDebitMark::Credit,
                funds_code: Some(FundsCode::try_from(&'K').unwrap()),
                amount: Amount::try_from("366336,2").unwrap(),
                transaction_type: TransactionType::NonSwiftTransfer,
                identification_code: IdentificationCode::try_from("TRF").unwrap(),
                account_owner_ref: AccountOwnerRef::try_from("Arbi/deposit").unwrap(),
                bank_ref: Some(BankRef::try_from("1323333800").unwrap()),
                information_to_account_owner: None,
                supplementary_details: None,
            })
        );
        assert_eq!(
            result.unwrap().to_string(),
            "- Value date: 2023-03-01\n- Entry date: 2023-02-28\n- Debit/Credit: Credit\n- Funds code: K\n- Amount: 366336.2\n- Transaction type: Non-SWIFT transfer\n- Identification code: TRF\n- Account owner reference: Arbi/deposit\n- Bank reference: 1323333800\n"
        );
    }

    #[test]
    fn test_add_supplementary_details() {
        let mut statement_line = StatementLine::try_from(DATA).unwrap();
        let result = statement_line.add_supplementary_details(
            SupplementaryDetails::try_from("Test supplementary details").unwrap(),
        );
        assert!(result.is_ok());
        assert_eq!(
            statement_line.to_string(),
            "- Value date: 2023-03-01\n- Entry date: 2023-02-28\n- Debit/Credit: Credit\n- Funds code: K\n- Amount: 366336.2\n- Transaction type: Non-SWIFT transfer\n- Identification code: TRF\n- Account owner reference: Arbi/deposit\n- Bank reference: 1323333800\n- Supplementary details: Test supplementary details\n"
        );

        let result = statement_line.add_supplementary_details(
            SupplementaryDetails::try_from("Test supplementary details 2").unwrap(),
        );
        assert_eq!(
            result,
            Err(StatementLineError::SupplementaryDetailsAlreadySet)
        );
        assert_eq!(
            result.unwrap_err().to_string(),
            "Supplementary details already set"
        );
    }

    #[test]
    fn test_add_information_to_account_owner() {
        let mut statement_line = StatementLine::try_from(DATA).unwrap();
        let _ = statement_line.add_information_to_account_owner(
            InformationToAccountOwner::try_from("Test information to account owner").unwrap(),
        );
        let _ = statement_line.add_information_to_account_owner(
            InformationToAccountOwner::try_from("Test information to account owner 2").unwrap(),
        );
        let _ = statement_line.add_information_to_account_owner(
            InformationToAccountOwner::try_from("Test information to account owner 3").unwrap(),
        );
        let _ = statement_line.add_information_to_account_owner(
            InformationToAccountOwner::try_from("Test information to account owner 4").unwrap(),
        );
        let _ = statement_line.add_information_to_account_owner(
            InformationToAccountOwner::try_from("Test information to account owner 5").unwrap(),
        );
        let result = statement_line.add_information_to_account_owner(
            InformationToAccountOwner::try_from("Test information to account owner 6").unwrap(),
        );
        assert!(result.is_ok());
        assert_eq!(
            statement_line.to_string(),
            "- Value date: 2023-03-01\n- Entry date: 2023-02-28\n- Debit/Credit: Credit\n- Funds code: K\n- Amount: 366336.2\n- Transaction type: Non-SWIFT transfer\n- Identification code: TRF\n- Account owner reference: Arbi/deposit\n- Bank reference: 1323333800\n- Information to account owner: Test information to account owner Test information to account owner 2 Test information to account owner 3 Test information to account owner 4 Test information to account owner 5 Test information to account owner 6\n"
        );

        let result = statement_line.add_information_to_account_owner(
            InformationToAccountOwner::try_from("Test information to account owner 7").unwrap(),
        );
        assert_eq!(
            result,
            Err(StatementLineError::InformationToAccountOwnerTooLong)
        );
        assert_eq!(
            result.unwrap_err().to_string(),
            "Information to account owner too long"
        );
    }
}
