use std::error::Error;
use std::fmt::{Display, Formatter};

const INFORMATION_TO_ACCOUNT_OWNER_MAX_LENGTH: usize = 65;

#[derive(Debug, PartialEq)]
pub(super) struct InformationToAccountOwner(String);

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
        Ok(Self(value.to_string()))
    }
}

impl Display for InformationToAccountOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

#[cfg(test)]
mod tests {
    use super::*;

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
            Ok(InformationToAccountOwner("ValidInformation".to_string()))
        );
        assert_eq!(result.unwrap().to_string(), "ValidInformation");
    }
}
