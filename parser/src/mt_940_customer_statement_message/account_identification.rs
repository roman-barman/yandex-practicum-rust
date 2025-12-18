use std::fmt::Display;
use std::io::Write;
use thiserror::Error;

const ACCOUNT_IDENTIFICATION_MAX_LENGTH: usize = 35;

#[derive(Debug, PartialEq)]
pub(crate) struct AccountIdentification(String);

impl AsRef<str> for AccountIdentification {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AccountIdentification {
    pub(super) fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.0.as_bytes())
    }
}

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
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Error)]
pub(crate) enum AccountIdentificationParseError {
    #[error("Account identification cannot be empty")]
    Empty,
    #[error(
        "Account identification cannot be longer than {} characters",
        ACCOUNT_IDENTIFICATION_MAX_LENGTH
    )]
    TooLong,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_account_identification_write_to() {
        let mut buffer = Cursor::new(Vec::new());
        AccountIdentification("12345DK".to_string())
            .write_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer.get_ref(), b"12345DK");
    }

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
                "Account identification cannot be longer than {} characters",
                ACCOUNT_IDENTIFICATION_MAX_LENGTH
            )
        );
    }

    #[test]
    fn test_valid_account_identification() {
        let result = AccountIdentification::try_from("12345DK");
        assert_eq!(result, Ok(AccountIdentification("12345DK".to_string())));
        assert_eq!(result.unwrap().to_string(), "12345DK");
    }
}
