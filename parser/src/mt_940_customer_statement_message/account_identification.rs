use std::error::Error;
use std::fmt::Display;

const ACCOUNT_IDENTIFICATION_MAX_LENGTH: usize = 35;

#[derive(Debug, PartialEq)]
pub(super) struct AccountIdentification(String);

impl TryFrom<&str> for AccountIdentification {
    type Error = AccountIdentificationParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(AccountIdentificationParseError::Empty);
        }
        if value.len() > ACCOUNT_IDENTIFICATION_MAX_LENGTH {
            return Err(AccountIdentificationParseError::TooLong);
        }
        Ok(AccountIdentification(value.to_string()))
    }
}

impl Display for AccountIdentification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Account identification: {}", self.0)
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum AccountIdentificationParseError {
    Empty,
    TooLong,
}

impl Display for AccountIdentificationParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountIdentificationParseError::Empty => {
                write!(f, "Account identification cannot be empty")
            }
            AccountIdentificationParseError::TooLong => write!(
                f,
                "Account identification exceeds maximum length of {} characters",
                ACCOUNT_IDENTIFICATION_MAX_LENGTH
            ),
        }
    }
}

impl Error for AccountIdentificationParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_account_identification() {
        let result = AccountIdentification::try_from("");
        assert_eq!(result, Err(AccountIdentificationParseError::Empty));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Account identification cannot be empty"
        );
    }

    #[test]
    fn test_long_account_identification() {
        let result = AccountIdentification::try_from(
            "1".repeat(ACCOUNT_IDENTIFICATION_MAX_LENGTH + 1).as_str(),
        );
        assert_eq!(result, Err(AccountIdentificationParseError::TooLong));
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Account identification exceeds maximum length of {} characters",
                ACCOUNT_IDENTIFICATION_MAX_LENGTH
            )
        );
    }

    #[test]
    fn test_valid_account_identification() {
        let result = AccountIdentification::try_from("12345DK");
        assert_eq!(result, Ok(AccountIdentification("12345DK".to_string())));
        assert_eq!(
            result.unwrap().to_string(),
            "Account identification: 12345DK"
        );
    }
}
