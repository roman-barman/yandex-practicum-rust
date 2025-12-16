mod account_identification;
mod amount;
pub(crate) mod balance;
pub(crate) mod date;
pub mod error;
mod information_to_account_owner;
mod related_reference;
pub(crate) mod statement_line;
pub(crate) mod statement_sequence_number;
pub(crate) mod transaction_reference_number;

use crate::MessageWriter;
use crate::camt_053_message::statement::Statement;
use crate::mt_940_customer_statement_message::account_identification::*;
use crate::mt_940_customer_statement_message::balance::state::*;
use crate::mt_940_customer_statement_message::balance::*;
use crate::mt_940_customer_statement_message::error::*;
use crate::mt_940_customer_statement_message::information_to_account_owner::*;
use crate::mt_940_customer_statement_message::related_reference::*;
use crate::mt_940_customer_statement_message::statement_line::supplementary_details::*;
use crate::mt_940_customer_statement_message::statement_line::*;
use crate::mt_940_customer_statement_message::statement_sequence_number::*;
use crate::mt_940_customer_statement_message::transaction_reference_number::*;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, Read, Write};

const TRANSACTION_REFERENCE_NUMBER_TAG: &str = ":20:";
const RELATED_REFERENCE_TAG: &str = ":21:";
const ACCOUNT_IDENTIFICATION_TAG: &str = ":25:";
const STATEMENT_SEQUENCE_NUMBER_TAG: &str = ":28C:";
const OPENING_F_BALANCE_TAG: &str = ":60F:";
const OPENING_M_BALANCE_TAG: &str = ":60M:";
const STATEMENT_LINE_TAG: &str = ":61:";
const CLOSING_F_BALANCE_TAG: &str = ":62F:";
const CLOSING_M_BALANCE_TAG: &str = ":62M:";
const INFO_TO_ACCOUNT_OWNER_TAG: &str = ":86:";
const STATEMENT_LINE_INFO_TO_ACCOUNT_OWNER_TAG: &str = ":86S";
const CLOSING_AVAILABLE_BALANCE_TAG: &str = ":64:";
const FORWARD_AVAILABLE_BALANCE_TAG: &str = ":65:";

pub struct Mt940CustomerStatementMessage {
    transaction_reference_number: TransactionReferenceNumber,
    related_reference: Option<RelatedReference>,
    account_identification: AccountIdentification,
    statement_sequence_no: StatementSequenceNumber,
    opening_balance: Balance,
    statement_lines: Option<Vec<StatementLine>>,
    closing_balance: Balance,
    closing_available_balance: Option<Balance>,
    forward_available_balance: Option<Balance>,
    information_to_account_owner: Option<InformationToAccountOwner>,
}

impl Display for Mt940CustomerStatementMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Transaction reference number: {}",
            self.transaction_reference_number
        )?;
        if let Some(ref related_reference) = self.related_reference {
            writeln!(f, "Related reference: {}", related_reference)?;
        }
        writeln!(f, "Account identification: {}", self.account_identification)?;
        writeln!(
            f,
            "Statement sequence number: {}",
            self.statement_sequence_no
        )?;
        writeln!(f, "Opening balance")?;
        write!(f, "{}", self.opening_balance)?;
        if let Some(ref statement_lines) = self.statement_lines {
            for statement_line in statement_lines {
                writeln!(f, "Statement line")?;
                write!(f, "{}", statement_line)?;
            }
        }
        writeln!(f, "Closing balance")?;
        write!(f, "{}", self.closing_balance)?;
        if let Some(ref closing_available_balance) = self.closing_available_balance {
            writeln!(f, "Closing available balance")?;
            write!(f, "{}", closing_available_balance)?;
        }
        if let Some(ref forward_available_balance) = self.forward_available_balance {
            writeln!(f, "Forward available balance")?;
            write!(f, "{}", forward_available_balance)?;
        }
        if let Some(ref information_to_account_owner) = self.information_to_account_owner {
            writeln!(f, "Information to account owner")?;
            write!(f, "{}", information_to_account_owner)?;
        }
        Ok(())
    }
}

impl TryFrom<&Statement> for Mt940CustomerStatementMessage {
    type Error = Mt940CustomerStatementMessageReadError;

    fn try_from(value: &Statement) -> Result<Self, Self::Error> {
        let mut builder = Mt940CustomerStatementMessageBuilder::default();
        builder.add_transaction_reference_number(TransactionReferenceNumber::try_from(
            value.get_identification(),
        )?);
        builder.add_account_identification(AccountIdentification::try_from(
            value.get_account_identification(),
        )?);
        let statement_sequence_number = if let Some(number) = value.get_legal_sequence_number() {
            StatementSequenceNumber::try_from(
                format!("{}/{}", value.get_electronic_sequence_number(), number).as_str(),
            )?
        } else {
            StatementSequenceNumber::try_from(
                format!("{}", value.get_electronic_sequence_number()).as_str(),
            )?
        };
        builder.add_statement_sequence_number(statement_sequence_number);
        builder.add_opening_balance(Balance::try_from(
            value
                .get_opening_balance()
                .ok_or(BalanceParseError::Empty)?,
        )?);
        builder.add_closing_balance(Balance::try_from(
            value
                .get_closing_balance()
                .ok_or(BalanceParseError::Empty)?,
        )?);

        for entry in value.get_entries().ok_or(StatementLineParseError::Empty)? {
            for statement_line in StatementLine::try_from_entry(entry)? {
                builder.add_statement_line(statement_line);
            }
        }

        builder.build()
    }
}

impl MessageWriter for Mt940CustomerStatementMessage {
    type Error = std::io::Error;
    fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), Self::Error> {
        write!(writer, "{}", TRANSACTION_REFERENCE_NUMBER_TAG)?;
        self.transaction_reference_number.write_to(writer)?;
        writeln!(writer)?;

        if let Some(ref related_reference) = self.related_reference {
            write!(writer, "{}", RELATED_REFERENCE_TAG)?;
            related_reference.write_to(writer)?;
            writeln!(writer)?;
        }

        write!(writer, "{}", ACCOUNT_IDENTIFICATION_TAG)?;
        self.account_identification.write_to(writer)?;
        writeln!(writer)?;

        write!(writer, "{}", STATEMENT_SEQUENCE_NUMBER_TAG)?;
        self.statement_sequence_no.write_to(writer)?;
        writeln!(writer)?;

        match self.opening_balance.get_state() {
            Some(State::Final) => write!(writer, "{}", OPENING_F_BALANCE_TAG)?,
            Some(State::Intermediate) => write!(writer, "{}", OPENING_M_BALANCE_TAG)?,
            None => write!(writer, "{}", OPENING_F_BALANCE_TAG)?,
        }
        self.opening_balance.write_to(writer)?;
        writeln!(writer)?;

        if let Some(ref statement_lines) = self.statement_lines {
            for statement_line in statement_lines {
                write!(writer, "{}", STATEMENT_LINE_TAG)?;
                statement_line.statement_line_write_to(writer)?;
                writeln!(writer)?;
                if let Some(info) = statement_line.get_information_to_account_owner() {
                    write!(writer, "{}", INFO_TO_ACCOUNT_OWNER_TAG)?;
                    info.write_to(writer)?;
                    writeln!(writer)?;
                }
            }
        }

        match self.closing_balance.get_state() {
            Some(State::Final) => write!(writer, "{}", CLOSING_F_BALANCE_TAG)?,
            Some(State::Intermediate) => write!(writer, "{}", CLOSING_M_BALANCE_TAG)?,
            None => write!(writer, "{}", CLOSING_F_BALANCE_TAG)?,
        }
        self.closing_balance.write_to(writer)?;
        writeln!(writer)?;

        if let Some(ref closing_available_balance) = self.closing_available_balance {
            write!(writer, "{}", CLOSING_AVAILABLE_BALANCE_TAG)?;
            closing_available_balance.write_to(writer)?;
            writeln!(writer)?;
        }

        if let Some(ref forward_available_balance) = self.forward_available_balance {
            write!(writer, "{}", FORWARD_AVAILABLE_BALANCE_TAG)?;
            forward_available_balance.write_to(writer)?;
            writeln!(writer)?;
        }

        if let Some(ref information_to_account_owner) = self.information_to_account_owner {
            write!(writer, "{}", INFO_TO_ACCOUNT_OWNER_TAG)?;
            information_to_account_owner.write_to(writer)?;
            writeln!(writer)?;
        }

        Ok(())
    }
}

impl Mt940CustomerStatementMessage {
    pub fn read_from<T: Read>(
        reader: T,
    ) -> Result<Vec<Self>, Mt940CustomerStatementMessageReadError> {
        let mut result = Vec::new();
        let buf_reader = BufReader::new(reader);
        let mut previous_tag = None;
        let mut builder = Mt940CustomerStatementMessageBuilder::default();
        for line in buf_reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let line = if is_tag(&line) {
                line.replace(" ", "")
            } else {
                line.to_string()
            };

            match previous_tag {
                None if !is_tag(&line) => continue,
                None if line.starts_with(TRANSACTION_REFERENCE_NUMBER_TAG) => {
                    read_transaction_reference_number(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                    )?
                }
                Some(TRANSACTION_REFERENCE_NUMBER_TAG) if !is_tag(&line) => continue,
                Some(TRANSACTION_REFERENCE_NUMBER_TAG)
                    if line.starts_with(RELATED_REFERENCE_TAG) =>
                {
                    builder.add_related_reference(RelatedReference::try_from(
                        line.trim_start_matches(RELATED_REFERENCE_TAG),
                    )?);
                    previous_tag = Some(RELATED_REFERENCE_TAG);
                }
                Some(TRANSACTION_REFERENCE_NUMBER_TAG)
                    if line.starts_with(ACCOUNT_IDENTIFICATION_TAG) =>
                {
                    read_account_identification(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                        ACCOUNT_IDENTIFICATION_TAG,
                    )?
                }
                Some(RELATED_REFERENCE_TAG) if !is_tag(&line) => continue,
                Some(RELATED_REFERENCE_TAG) if line.starts_with(ACCOUNT_IDENTIFICATION_TAG) => {
                    read_account_identification(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                        ACCOUNT_IDENTIFICATION_TAG,
                    )?;
                }
                Some(ACCOUNT_IDENTIFICATION_TAG) if !is_tag(&line) => {
                    continue;
                }
                Some(ACCOUNT_IDENTIFICATION_TAG)
                    if line.starts_with(STATEMENT_SEQUENCE_NUMBER_TAG) =>
                {
                    builder.add_statement_sequence_number(StatementSequenceNumber::try_from(
                        line.trim_start_matches(STATEMENT_SEQUENCE_NUMBER_TAG),
                    )?);
                    previous_tag = Some(STATEMENT_SEQUENCE_NUMBER_TAG);
                }
                Some(STATEMENT_SEQUENCE_NUMBER_TAG) if !is_tag(&line) => continue,
                Some(STATEMENT_SEQUENCE_NUMBER_TAG) if line.starts_with(OPENING_F_BALANCE_TAG) => {
                    read_opening_balance(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                        OPENING_F_BALANCE_TAG,
                        State::Final,
                    )?;
                }
                Some(STATEMENT_SEQUENCE_NUMBER_TAG) if line.starts_with(OPENING_M_BALANCE_TAG) => {
                    read_opening_balance(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                        OPENING_M_BALANCE_TAG,
                        State::Intermediate,
                    )?;
                }
                Some(OPENING_F_BALANCE_TAG) | Some(OPENING_M_BALANCE_TAG) if !is_tag(&line) => {
                    continue;
                }
                Some(OPENING_F_BALANCE_TAG) | Some(OPENING_M_BALANCE_TAG)
                    if line.starts_with(STATEMENT_LINE_TAG) =>
                {
                    read_statement_line(line.as_str(), &mut previous_tag, &mut builder)?;
                }
                Some(OPENING_F_BALANCE_TAG) | Some(OPENING_M_BALANCE_TAG)
                    if line.starts_with(CLOSING_F_BALANCE_TAG) =>
                {
                    read_closing_balance(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                        CLOSING_F_BALANCE_TAG,
                        State::Final,
                    )?;
                }
                Some(OPENING_F_BALANCE_TAG) | Some(OPENING_M_BALANCE_TAG)
                    if line.starts_with(CLOSING_M_BALANCE_TAG) =>
                {
                    read_closing_balance(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                        CLOSING_M_BALANCE_TAG,
                        State::Intermediate,
                    )?;
                }
                Some(STATEMENT_LINE_TAG) if !is_tag(&line) => {
                    builder.add_supplementary_details(SupplementaryDetails::try_from(
                        line.as_str(),
                    )?)?;
                }
                Some(STATEMENT_LINE_TAG) if line.starts_with(STATEMENT_LINE_TAG) => {
                    read_statement_line(line.as_str(), &mut previous_tag, &mut builder)?;
                }
                Some(STATEMENT_LINE_TAG) if line.starts_with(INFO_TO_ACCOUNT_OWNER_TAG) => {
                    builder.add_statement_line_info_to_account_owner(
                        InformationToAccountOwner::try_from(
                            line.trim_start_matches(INFO_TO_ACCOUNT_OWNER_TAG),
                        )?,
                    )?;
                    previous_tag = Some(STATEMENT_LINE_INFO_TO_ACCOUNT_OWNER_TAG);
                }
                Some(STATEMENT_LINE_TAG) if line.starts_with(CLOSING_F_BALANCE_TAG) => {
                    read_closing_balance(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                        CLOSING_F_BALANCE_TAG,
                        State::Final,
                    )?;
                }
                Some(STATEMENT_LINE_TAG) if line.starts_with(CLOSING_M_BALANCE_TAG) => {
                    read_closing_balance(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                        CLOSING_M_BALANCE_TAG,
                        State::Intermediate,
                    )?;
                }
                Some(STATEMENT_LINE_INFO_TO_ACCOUNT_OWNER_TAG) if !is_tag(&line) => {
                    builder.add_statement_line_info_to_account_owner(
                        InformationToAccountOwner::try_from(line.as_str())?,
                    )?;
                }
                Some(STATEMENT_LINE_INFO_TO_ACCOUNT_OWNER_TAG)
                    if line.starts_with(STATEMENT_LINE_TAG) =>
                {
                    read_statement_line(line.as_str(), &mut previous_tag, &mut builder)?;
                }
                Some(STATEMENT_LINE_INFO_TO_ACCOUNT_OWNER_TAG)
                    if line.starts_with(CLOSING_F_BALANCE_TAG) =>
                {
                    read_closing_balance(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                        CLOSING_F_BALANCE_TAG,
                        State::Final,
                    )?;
                }
                Some(STATEMENT_LINE_INFO_TO_ACCOUNT_OWNER_TAG)
                    if line.starts_with(CLOSING_M_BALANCE_TAG) =>
                {
                    read_closing_balance(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                        CLOSING_M_BALANCE_TAG,
                        State::Intermediate,
                    )?;
                }
                Some(CLOSING_F_BALANCE_TAG) | Some(CLOSING_M_BALANCE_TAG) if !is_tag(&line) => {
                    continue;
                }
                Some(CLOSING_F_BALANCE_TAG) | Some(CLOSING_M_BALANCE_TAG)
                    if line.starts_with(CLOSING_AVAILABLE_BALANCE_TAG) =>
                {
                    builder.add_closing_available_balance(Balance::try_from(
                        line.trim_start_matches(CLOSING_AVAILABLE_BALANCE_TAG),
                    )?);
                    previous_tag = Some(CLOSING_AVAILABLE_BALANCE_TAG);
                }
                Some(CLOSING_F_BALANCE_TAG) | Some(CLOSING_M_BALANCE_TAG)
                    if line.starts_with(FORWARD_AVAILABLE_BALANCE_TAG) =>
                {
                    read_forward_available_balance(line.as_str(), &mut previous_tag, &mut builder)?;
                }
                Some(CLOSING_F_BALANCE_TAG) | Some(CLOSING_M_BALANCE_TAG)
                    if line.starts_with(INFO_TO_ACCOUNT_OWNER_TAG) =>
                {
                    read_information_to_account_owner(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                    )?;
                }
                Some(CLOSING_F_BALANCE_TAG) | Some(CLOSING_M_BALANCE_TAG)
                    if line.starts_with(TRANSACTION_REFERENCE_NUMBER_TAG) =>
                {
                    let message = builder.build()?;
                    result.push(message);
                    builder = Mt940CustomerStatementMessageBuilder::default();
                    read_transaction_reference_number(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                    )?;
                }
                Some(CLOSING_AVAILABLE_BALANCE_TAG) if !is_tag(&line) => continue,
                Some(CLOSING_AVAILABLE_BALANCE_TAG)
                    if line.starts_with(FORWARD_AVAILABLE_BALANCE_TAG) =>
                {
                    read_forward_available_balance(line.as_str(), &mut previous_tag, &mut builder)?;
                }
                Some(CLOSING_AVAILABLE_BALANCE_TAG)
                    if line.starts_with(INFO_TO_ACCOUNT_OWNER_TAG) =>
                {
                    read_information_to_account_owner(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                    )?;
                }
                Some(CLOSING_AVAILABLE_BALANCE_TAG)
                    if line.starts_with(TRANSACTION_REFERENCE_NUMBER_TAG) =>
                {
                    let message = builder.build()?;
                    result.push(message);
                    builder = Mt940CustomerStatementMessageBuilder::default();
                    read_transaction_reference_number(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                    )?;
                }
                Some(FORWARD_AVAILABLE_BALANCE_TAG) if !is_tag(&line) => continue,
                Some(FORWARD_AVAILABLE_BALANCE_TAG)
                    if line.starts_with(INFO_TO_ACCOUNT_OWNER_TAG) =>
                {
                    read_information_to_account_owner(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                    )?;
                }
                Some(FORWARD_AVAILABLE_BALANCE_TAG)
                    if line.starts_with(TRANSACTION_REFERENCE_NUMBER_TAG) =>
                {
                    let message = builder.build()?;
                    result.push(message);
                    builder = Mt940CustomerStatementMessageBuilder::default();
                    read_transaction_reference_number(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                    )?;
                }
                Some(INFO_TO_ACCOUNT_OWNER_TAG) if !is_tag(&line) => {
                    builder.add_information_to_account_owner(
                        InformationToAccountOwner::try_from(line.as_str())?,
                    )?;
                }
                Some(INFO_TO_ACCOUNT_OWNER_TAG)
                    if line.starts_with(TRANSACTION_REFERENCE_NUMBER_TAG) =>
                {
                    let message = builder.build()?;
                    result.push(message);
                    builder = Mt940CustomerStatementMessageBuilder::default();
                    read_transaction_reference_number(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                    )?;
                }
                _ => return Err(Mt940CustomerStatementMessageReadError::invalid_format()),
            }
        }

        let message = builder.build()?;
        result.push(message);

        Ok(result)
    }

    pub(crate) fn get_transaction_reference_number(&self) -> &TransactionReferenceNumber {
        &self.transaction_reference_number
    }

    pub(crate) fn get_statement_sequence_number(&self) -> &StatementSequenceNumber {
        &self.statement_sequence_no
    }

    pub(crate) fn get_account_identification(&self) -> &AccountIdentification {
        &self.account_identification
    }

    pub(crate) fn get_opening_balance(&self) -> &Balance {
        &self.opening_balance
    }

    pub(crate) fn get_closing_balance(&self) -> &Balance {
        &self.closing_balance
    }

    pub(crate) fn get_statement_lines(&self) -> Option<&[StatementLine]> {
        self.statement_lines.as_deref()
    }
}

fn read_information_to_account_owner(
    line: &str,
    previous_tag: &mut Option<&str>,
    builder: &mut Mt940CustomerStatementMessageBuilder,
) -> Result<(), Mt940CustomerStatementMessageReadError> {
    builder.add_information_to_account_owner(InformationToAccountOwner::try_from(
        line.trim_start_matches(INFO_TO_ACCOUNT_OWNER_TAG),
    )?)?;
    *previous_tag = Some(INFO_TO_ACCOUNT_OWNER_TAG);
    Ok(())
}

fn read_forward_available_balance(
    line: &str,
    previous_tag: &mut Option<&str>,
    builder: &mut Mt940CustomerStatementMessageBuilder,
) -> Result<(), Mt940CustomerStatementMessageReadError> {
    builder.add_forward_available_balance(Balance::try_from(
        line.trim_start_matches(FORWARD_AVAILABLE_BALANCE_TAG),
    )?);
    *previous_tag = Some(FORWARD_AVAILABLE_BALANCE_TAG);
    Ok(())
}

fn read_closing_balance(
    line: &str,
    previous_tag: &mut Option<&str>,
    builder: &mut Mt940CustomerStatementMessageBuilder,
    tag: &'static str,
    state: State,
) -> Result<(), Mt940CustomerStatementMessageReadError> {
    let mut balance = Balance::try_from(line.trim_start_matches(tag))?;
    balance.set_state(state);
    builder.add_closing_balance(balance);
    *previous_tag = Some(tag);
    Ok(())
}

fn read_statement_line(
    line: &str,
    previous_tag: &mut Option<&str>,
    builder: &mut Mt940CustomerStatementMessageBuilder,
) -> Result<(), Mt940CustomerStatementMessageReadError> {
    builder.add_statement_line(StatementLine::try_from(
        line.trim_start_matches(STATEMENT_LINE_TAG),
    )?);
    *previous_tag = Some(STATEMENT_LINE_TAG);
    Ok(())
}

fn read_opening_balance(
    line: &str,
    previous_tag: &mut Option<&str>,
    builder: &mut Mt940CustomerStatementMessageBuilder,
    tag: &'static str,
    balance_state: State,
) -> Result<(), Mt940CustomerStatementMessageReadError> {
    let mut balance = Balance::try_from(line.trim_start_matches(tag))?;
    balance.set_state(balance_state);
    builder.add_opening_balance(balance);
    *previous_tag = Some(tag);
    Ok(())
}

fn read_transaction_reference_number(
    line: &str,
    previous_tag: &mut Option<&str>,
    builder: &mut Mt940CustomerStatementMessageBuilder,
) -> Result<(), Mt940CustomerStatementMessageReadError> {
    builder.add_transaction_reference_number(TransactionReferenceNumber::try_from(
        line.trim_start_matches(TRANSACTION_REFERENCE_NUMBER_TAG),
    )?);
    *previous_tag = Some(TRANSACTION_REFERENCE_NUMBER_TAG);
    Ok(())
}

fn read_account_identification(
    line: &str,
    previous_tag: &mut Option<&str>,
    builder: &mut Mt940CustomerStatementMessageBuilder,
    tag: &'static str,
) -> Result<(), Mt940CustomerStatementMessageReadError> {
    builder.add_account_identification(AccountIdentification::try_from(
        line.trim_start_matches(tag),
    )?);
    *previous_tag = Some(tag);
    Ok(())
}

fn is_tag(line: &impl AsRef<str>) -> bool {
    line.as_ref().starts_with(":")
}

#[derive(Default)]
struct Mt940CustomerStatementMessageBuilder {
    transaction_reference_number: Option<TransactionReferenceNumber>,
    related_reference: Option<RelatedReference>,
    account_identification: Option<AccountIdentification>,
    statement_sequence_no: Option<StatementSequenceNumber>,
    opening_balance: Option<Balance>,
    statement_lines: Option<Vec<StatementLine>>,
    closing_balance: Option<Balance>,
    closing_available_balance: Option<Balance>,
    forward_available_balance: Option<Balance>,
    information_to_account_owner: Option<InformationToAccountOwner>,
}

impl Mt940CustomerStatementMessageBuilder {
    fn add_transaction_reference_number(&mut self, value: TransactionReferenceNumber) {
        self.transaction_reference_number = Some(value);
    }

    fn add_related_reference(&mut self, value: RelatedReference) {
        self.related_reference = Some(value);
    }

    fn add_account_identification(&mut self, value: AccountIdentification) {
        self.account_identification = Some(value);
    }

    fn add_statement_sequence_number(&mut self, value: StatementSequenceNumber) {
        self.statement_sequence_no = Some(value);
    }

    fn add_opening_balance(&mut self, value: Balance) {
        self.opening_balance = Some(value);
    }

    fn add_statement_line(&mut self, value: StatementLine) {
        match self.statement_lines {
            Some(ref mut statement_lines) => {
                statement_lines.push(value);
            }
            None => {
                self.statement_lines = Some(vec![value]);
            }
        }
    }

    fn add_closing_balance(&mut self, value: Balance) {
        self.closing_balance = Some(value);
    }

    fn add_supplementary_details(
        &mut self,
        value: SupplementaryDetails,
    ) -> Result<(), Mt940CustomerStatementMessageReadError> {
        match self.statement_lines {
            Some(ref mut statement_lines) => {
                statement_lines
                    .last_mut()
                    .unwrap()
                    .add_supplementary_details(value)?;
            }
            None => {
                return Err(Mt940CustomerStatementMessageReadError::invalid_format());
            }
        }
        Ok(())
    }

    fn add_statement_line_info_to_account_owner(
        &mut self,
        value: InformationToAccountOwner,
    ) -> Result<(), Mt940CustomerStatementMessageReadError> {
        match self.statement_lines {
            Some(ref mut statement_lines) => {
                statement_lines
                    .last_mut()
                    .unwrap()
                    .add_information_to_account_owner(value)?;
            }
            None => {
                return Err(Mt940CustomerStatementMessageReadError::invalid_format());
            }
        }
        Ok(())
    }

    fn add_closing_available_balance(&mut self, value: Balance) {
        self.closing_available_balance = Some(value);
    }

    fn add_forward_available_balance(&mut self, value: Balance) {
        self.forward_available_balance = Some(value);
    }

    fn add_information_to_account_owner(
        &mut self,
        value: InformationToAccountOwner,
    ) -> Result<(), Mt940CustomerStatementMessageReadError> {
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

    fn build(
        self,
    ) -> Result<Mt940CustomerStatementMessage, Mt940CustomerStatementMessageReadError> {
        if self.transaction_reference_number.is_none()
            || self.account_identification.is_none()
            || self.statement_sequence_no.is_none()
            || self.opening_balance.is_none()
            || self.closing_balance.is_none()
        {
            return Err(Mt940CustomerStatementMessageReadError::invalid_format());
        }

        Ok(Mt940CustomerStatementMessage {
            transaction_reference_number: self.transaction_reference_number.unwrap(),
            related_reference: self.related_reference,
            account_identification: self.account_identification.unwrap(),
            statement_sequence_no: self.statement_sequence_no.unwrap(),
            opening_balance: self.opening_balance.unwrap(),
            statement_lines: self.statement_lines,
            closing_balance: self.closing_balance.unwrap(),
            closing_available_balance: self.closing_available_balance,
            forward_available_balance: self.forward_available_balance,
            information_to_account_owner: self.information_to_account_owner,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    const DATA: &str = "
{1:F01GSCRUS30XXXX3614000002}{2:I940GSCRUS30XXXXN}{4:
:20:15486025400
:25:107048825
:28C:49/2
:60M:C250218USD2732398848,02
:61:2502180218D12,01NTRFGSLNVSHSUTKWDR//GI2504900007841
:86:/EREF/GSLNVSHSUTKWDR
/CRNM/GOLDMAN SACHS BANK USA
/CACT/107045863/CBIC/GSCRUS30XXX
/REMI/USD Payment to Vendor
/OPRP/Tag Payment
:61:2502180218D12,01NTRFGSOXWBAQYTF4VH//GI2504900005623
:86:/EREF/GSOXWBAQYTF4VH
/CRNM/GOLDMAN SACHS BANK USA
/CACT/107045863/CBIC/GSCRUS30XXX
/REMI/The maximum length of the block is 65 characters
/OPRP/Tag Payment
:61:2502180218D12,01NTRFGSC7MZKHS3UA23//GI2504900005621
:86:/EREF/GSC7MZKHS3UA23
/CRNM/GOLDMAN SACHS BANK USA
/CACT/107045863/CBIC/GSCRUS30XXX
/REMI/USD Payment from USD account
/OPRP/Tag Payment
:61:2502180218C11,25NTRFGS0DUTB31IOUHRS//GI2504900004512
:86:/EREF/GS0DUTB31IOUHRS
/DACT/8348577826/DBIC/CITIUS30XXX
/OAMT/11-25/
/DCID/CPQYTB74
:62M:C250218USD2937898,77
-}";

    #[test]
    fn test_write_to() {
        let mut cursor = Cursor::new(DATA.as_bytes());
        let message = Mt940CustomerStatementMessage::read_from(&mut cursor)
            .unwrap()
            .pop()
            .unwrap();

        let mut buffer = Cursor::new(Vec::new());
        message.write_to(&mut buffer).unwrap();
        assert_eq!(buffer.get_ref(), DATA[55..896].as_bytes());
    }

    #[test]
    fn test_read_from() {
        let mut cursor = Cursor::new(DATA.as_bytes());
        let result = Mt940CustomerStatementMessage::read_from(&mut cursor);
        assert!(result.is_ok());

        let mut messages = result.unwrap();
        assert_eq!(messages.len(), 1);

        let message = messages.pop().unwrap();
        assert_eq!(
            message.transaction_reference_number,
            TransactionReferenceNumber::try_from("15486025400").unwrap()
        );
        assert_eq!(message.related_reference, None);
        assert_eq!(
            message.account_identification,
            AccountIdentification::try_from("107048825").unwrap()
        );
        assert_eq!(
            message.statement_sequence_no,
            StatementSequenceNumber::try_from("49/2").unwrap()
        );
        let mut opening_balance = Balance::try_from("C250218USD2732398848,02").unwrap();
        opening_balance.set_state(State::Intermediate);
        assert_eq!(message.opening_balance, opening_balance);
        assert_eq!(message.statement_lines.unwrap().len(), 4);
        let mut closing_balance = Balance::try_from("C250218USD2937898,77").unwrap();
        closing_balance.set_state(State::Intermediate);
        assert_eq!(message.closing_balance, closing_balance);
        assert_eq!(message.closing_available_balance, None);
        assert_eq!(message.forward_available_balance, None);
        assert_eq!(message.information_to_account_owner, None);
    }
}
