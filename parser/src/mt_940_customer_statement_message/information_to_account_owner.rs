use crate::camt_053_message::statement::entry::bank_transaction_code::BankTransactionCode;
use crate::camt_053_message::statement::entry::entry_details::TransactionDetails;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use thiserror::Error;

const INFORMATION_TO_ACCOUNT_OWNER_MAX_LENGTH: usize = 65;
const INFORMATION_MAX_LENGTH: usize = 6;

#[derive(Debug, PartialEq, Clone)]
pub(super) struct InformationToAccountOwner(Vec<String>);

impl InformationToAccountOwner {
    pub(super) fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        for (i, line) in self.0.iter().enumerate() {
            if i != self.0.len() - 1 {
                writeln!(writer, "{}", line)?;
            } else {
                write!(writer, "{}", line)?;
            }
        }
        Ok(())
    }

    pub(super) fn try_add(
        &mut self,
        value: InformationToAccountOwner,
    ) -> Result<(), InformationToAccountOwnerError> {
        if self.0.len() + value.0.len() > INFORMATION_MAX_LENGTH {
            Err(InformationToAccountOwnerError::TooLong)
        } else {
            self.0.extend(value.0);
            Ok(())
        }
    }

    pub(super) fn from_bank_transaction_code(
        value: &BankTransactionCode,
    ) -> Option<InformationToAccountOwner> {
        if let Some(code) = value.get_proprietary_code() {
            if code.len() >= INFORMATION_TO_ACCOUNT_OWNER_MAX_LENGTH {
                Some(InformationToAccountOwner(vec![code.to_string()]))
            } else {
                Some(split_text_to_information_to_account_owner(code))
            }
        } else {
            None
        }
    }

    pub(super) fn from_transaction_details(
        transaction_details: &TransactionDetails,
    ) -> Option<InformationToAccountOwner> {
        let mut result: Option<InformationToAccountOwner> = None;

        let mut try_append = |text: &str| {
            let mut parsed = split_text_to_information_to_account_owner(text);
            let acc = result.get_or_insert(InformationToAccountOwner(Vec::new()));

            let available_slots = INFORMATION_MAX_LENGTH.saturating_sub(acc.0.len());
            if parsed.0.len() < available_slots {
                acc.0.append(&mut parsed.0);
            }
        };

        if let Some(unstructured) = transaction_details
            .get_remittance_information()
            .and_then(|ri| ri.get_unstructured())
        {
            for text in unstructured {
                try_append(text.as_str());
            }
        }

        if let Some(additional_information) = transaction_details.get_additional_information() {
            try_append(additional_information.as_str());
        }

        result
    }
}

fn split_text_to_information_to_account_owner(value: &str) -> InformationToAccountOwner {
    if value.len() >= INFORMATION_TO_ACCOUNT_OWNER_MAX_LENGTH {
        InformationToAccountOwner(vec![value.to_string()])
    } else {
        let mut info = String::new();
        let mut lines = vec![];
        for text in value.split_ascii_whitespace() {
            if info.len() + text.len() > INFORMATION_TO_ACCOUNT_OWNER_MAX_LENGTH {
                if lines.len() < INFORMATION_MAX_LENGTH {
                    lines.push(info);
                    info = text.to_string();
                } else {
                    return InformationToAccountOwner(lines);
                }
            } else {
                info.push_str(text);
            }
        }
        lines.push(info);
        InformationToAccountOwner(lines)
    }
}

impl Add for InformationToAccountOwner {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() + rhs.0.len() > INFORMATION_MAX_LENGTH {
            let mut result = vec![];
            result.extend(self.0);
            result.extend(rhs.0);
            Self(result)
        } else {
            self
        }
    }
}

impl TryFrom<&str> for InformationToAccountOwner {
    type Error = InformationToAccountOwnerParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(InformationToAccountOwnerParseError::Empty);
        }
        if value.len() > INFORMATION_TO_ACCOUNT_OWNER_MAX_LENGTH {
            return Err(InformationToAccountOwnerParseError::TooLong);
        }
        Ok(Self(vec![value.to_string()]))
    }
}

impl Display for InformationToAccountOwner {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.join(" "))
    }
}

#[derive(Debug, PartialEq, Error)]
pub(super) enum InformationToAccountOwnerParseError {
    #[error("Information to account owner cannot be empty")]
    Empty,
    #[error(
        "Information to account owner cannot be longer than {} characters",
        INFORMATION_TO_ACCOUNT_OWNER_MAX_LENGTH
    )]
    TooLong,
}

#[derive(Debug, PartialEq)]
pub(super) enum InformationToAccountOwnerError {
    TooLong,
}

impl Display for InformationToAccountOwnerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InformationToAccountOwnerError::TooLong => write!(
                f,
                "Information to account owner cannot be longer than {} lines",
                INFORMATION_MAX_LENGTH
            ),
        }
    }
}

impl Error for InformationToAccountOwnerError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_information_to_account_owner_write_to() {
        let mut buffer = Cursor::new(Vec::new());
        InformationToAccountOwner(vec![
            "ValidInformation".to_string(),
            "ValidInformation2".to_string(),
        ])
        .write_to(&mut buffer)
        .unwrap();
        assert_eq!(buffer.get_ref(), b"ValidInformation\nValidInformation2");
    }

    #[test]
    fn test_empty_information_to_account_owner() {
        let result = InformationToAccountOwner::try_from("");
        assert_eq!(result, Err(InformationToAccountOwnerParseError::Empty));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Information to account owner cannot be empty"
        );
    }

    #[test]
    fn test_long_information_to_account_owner() {
        let result = InformationToAccountOwner::try_from(
            "1".repeat(INFORMATION_TO_ACCOUNT_OWNER_MAX_LENGTH + 1)
                .as_str(),
        );
        assert_eq!(result, Err(InformationToAccountOwnerParseError::TooLong));
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Information to account owner cannot be longer than {} characters",
                INFORMATION_TO_ACCOUNT_OWNER_MAX_LENGTH
            )
        );
    }

    #[test]
    fn test_valid_information_to_account_owner() {
        let result = InformationToAccountOwner::try_from("ValidInformation");
        assert_eq!(
            result,
            Ok(InformationToAccountOwner(vec![
                "ValidInformation".to_string()
            ]))
        );
        assert_eq!(result.unwrap().to_string(), "ValidInformation");
    }

    #[test]
    fn test_add_information_to_account_owner() {
        let mut information_to_account_owner =
            InformationToAccountOwner::try_from("ValidInformation").unwrap();
        let result = information_to_account_owner
            .try_add(InformationToAccountOwner::try_from("ValidInformation2").unwrap());
        assert_eq!(result, Ok(()));
        assert_eq!(
            information_to_account_owner.to_string(),
            "ValidInformation ValidInformation2"
        );
    }

    #[test]
    fn test_add_information_to_account_owner_too_long() {
        let mut information_to_account_owner =
            InformationToAccountOwner::try_from("ValidInformation").unwrap();
        let result = information_to_account_owner
            .try_add(InformationToAccountOwner::try_from("ValidInformation2").unwrap());
        assert_eq!(result, Ok(()));

        let result = information_to_account_owner
            .try_add(InformationToAccountOwner::try_from("ValidInformation3").unwrap());
        assert_eq!(result, Ok(()));

        let result = information_to_account_owner
            .try_add(InformationToAccountOwner::try_from("ValidInformation4").unwrap());
        assert_eq!(result, Ok(()));

        let result = information_to_account_owner
            .try_add(InformationToAccountOwner::try_from("ValidInformation5").unwrap());
        assert_eq!(result, Ok(()));

        let result = information_to_account_owner
            .try_add(InformationToAccountOwner::try_from("ValidInformation6").unwrap());
        assert_eq!(result, Ok(()));

        let result = information_to_account_owner
            .try_add(InformationToAccountOwner::try_from("ValidInformation7").unwrap());
        assert_eq!(result, Err(InformationToAccountOwnerError::TooLong));
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Information to account owner cannot be longer than {} lines",
                INFORMATION_MAX_LENGTH
            )
        );
    }
}
