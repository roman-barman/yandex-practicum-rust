mod account_identification;
mod amount;
mod balance;
mod date;
pub mod error;
mod information_to_account_owner;
mod related_reference;
mod statement_line;
mod statement_sequence_number;
mod transaction_reference_number;

use crate::mt_940_customer_statement_message::account_identification::*;
use crate::mt_940_customer_statement_message::balance::*;
use crate::mt_940_customer_statement_message::error::*;
use crate::mt_940_customer_statement_message::information_to_account_owner::*;
use crate::mt_940_customer_statement_message::related_reference::*;
use crate::mt_940_customer_statement_message::statement_line::supplementary_details::*;
use crate::mt_940_customer_statement_message::statement_line::*;
use crate::mt_940_customer_statement_message::statement_sequence_number::*;
use crate::mt_940_customer_statement_message::transaction_reference_number::*;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, Read};

const TRANSACTION_REFERENCE_NUMBER_TAG: &str = ":20:";
const RELATED_REFERENCE_TAG: &str = ":21:";
const ACCOUNT_IDENTIFICATION_TAG: &str = ":25:";
const ACCOUNT_P_IDENTIFICATION_TAG: &str = ":25P:";
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
                Some(TRANSACTION_REFERENCE_NUMBER_TAG)
                    if line.starts_with(ACCOUNT_P_IDENTIFICATION_TAG) =>
                {
                    read_account_identification(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                        ACCOUNT_P_IDENTIFICATION_TAG,
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
                Some(RELATED_REFERENCE_TAG) if line.starts_with(ACCOUNT_P_IDENTIFICATION_TAG) => {
                    read_account_identification(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                        ACCOUNT_P_IDENTIFICATION_TAG,
                    )?;
                }
                Some(ACCOUNT_IDENTIFICATION_TAG) | Some(ACCOUNT_P_IDENTIFICATION_TAG)
                    if !is_tag(&line) =>
                {
                    continue;
                }
                Some(ACCOUNT_IDENTIFICATION_TAG) | Some(ACCOUNT_P_IDENTIFICATION_TAG)
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
                    )?;
                }
                Some(STATEMENT_SEQUENCE_NUMBER_TAG) if line.starts_with(OPENING_M_BALANCE_TAG) => {
                    read_opening_balance(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                        OPENING_M_BALANCE_TAG,
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
                    )?;
                }
                Some(STATEMENT_LINE_TAG) if line.starts_with(CLOSING_M_BALANCE_TAG) => {
                    read_closing_balance(
                        line.as_str(),
                        &mut previous_tag,
                        &mut builder,
                        CLOSING_M_BALANCE_TAG,
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
) -> Result<(), Mt940CustomerStatementMessageReadError> {
    builder.add_closing_balance(Balance::try_from(line.trim_start_matches(tag))?);
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
) -> Result<(), Mt940CustomerStatementMessageReadError> {
    builder.add_opening_balance(Balance::try_from(line.trim_start_matches(tag))?);
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
                information_to_account_owner.add(value)?;
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

    #[test]
    fn test_read_from() {
        let data = b"
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
        let mut cursor = Cursor::new(data);
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
        assert_eq!(
            message.opening_balance,
            Balance::try_from("C250218USD2732398848,02").unwrap()
        );
        assert_eq!(message.statement_lines.unwrap().len(), 4);
        assert_eq!(
            message.closing_balance,
            Balance::try_from("C250218USD2937898,77").unwrap()
        );
        assert_eq!(message.closing_available_balance, None);
        assert_eq!(message.forward_available_balance, None);
        assert_eq!(message.information_to_account_owner, None);
    }
}
