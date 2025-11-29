use std::error::Error;
use std::fmt::{Display, Formatter};

const INFORMATION_TO_ACCOUNT_OWNER_MAX_LENGTH: usize = 65;
const INFORMATION_MAX_LENGTH: usize = 6;

#[derive(Debug, PartialEq)]
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
}

impl InformationToAccountOwner {
    pub(super) fn add(
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

#[derive(Debug, PartialEq)]
pub(super) enum InformationToAccountOwnerParseError {
    Empty,
    TooLong,
}

impl Display for InformationToAccountOwnerParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            InformationToAccountOwnerParseError::Empty => {
                write!(f, "Information to account owner cannot be empty")
            }
            InformationToAccountOwnerParseError::TooLong => write!(
                f,
                "Information to account owner cannot be longer than {} characters",
                INFORMATION_TO_ACCOUNT_OWNER_MAX_LENGTH
            ),
        }
    }
}

impl Error for InformationToAccountOwnerParseError {}

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
            .add(InformationToAccountOwner::try_from("ValidInformation2").unwrap());
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
            .add(InformationToAccountOwner::try_from("ValidInformation2").unwrap());
        assert_eq!(result, Ok(()));

        let result = information_to_account_owner
            .add(InformationToAccountOwner::try_from("ValidInformation3").unwrap());
        assert_eq!(result, Ok(()));

        let result = information_to_account_owner
            .add(InformationToAccountOwner::try_from("ValidInformation4").unwrap());
        assert_eq!(result, Ok(()));

        let result = information_to_account_owner
            .add(InformationToAccountOwner::try_from("ValidInformation5").unwrap());
        assert_eq!(result, Ok(()));

        let result = information_to_account_owner
            .add(InformationToAccountOwner::try_from("ValidInformation6").unwrap());
        assert_eq!(result, Ok(()));

        let result = information_to_account_owner
            .add(InformationToAccountOwner::try_from("ValidInformation7").unwrap());
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
