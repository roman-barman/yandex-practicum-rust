use std::fmt::Display;
use thiserror::Error;

const ACCOUNT_OWNER_REF_MAX_LENGTH: usize = 16;

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct AccountOwnerRef(String);

impl AsRef<str> for AccountOwnerRef {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AccountOwnerRef {
    pub(super) fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.0.as_bytes())?;
        Ok(())
    }
}

impl TryFrom<&str> for AccountOwnerRef {
    type Error = AccountOwnerRefParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(AccountOwnerRefParseError::Empty);
        }
        if value.len() > ACCOUNT_OWNER_REF_MAX_LENGTH {
            return Err(AccountOwnerRefParseError::TooLong);
        }
        Ok(Self(value.to_string()))
    }
}

impl Display for AccountOwnerRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Error)]
pub(crate) enum AccountOwnerRefParseError {
    #[error("Account owner ref cannot be empty")]
    Empty,
    #[error(
        "Account owner ref cannot be longer than {} characters",
        ACCOUNT_OWNER_REF_MAX_LENGTH
    )]
    TooLong,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_account_owner_ref_write_to() {
        let mut buffer = Cursor::new(Vec::new());
        AccountOwnerRef("ValidRef".to_string())
            .write_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer.get_ref(), b"ValidRef");
    }

    #[test]
    fn test_empty_account_owner_ref() {
        let result = AccountOwnerRef::try_from("");
        assert_eq!(result, Err(AccountOwnerRefParseError::Empty));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Account owner ref cannot be empty"
        );
    }

    #[test]
    fn test_long_account_owner_ref() {
        let result =
            AccountOwnerRef::try_from("1".repeat(ACCOUNT_OWNER_REF_MAX_LENGTH + 1).as_str());
        assert_eq!(result, Err(AccountOwnerRefParseError::TooLong));
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Account owner ref cannot be longer than {} characters",
                ACCOUNT_OWNER_REF_MAX_LENGTH
            )
        );
    }

    #[test]
    fn test_valid_account_owner_ref() {
        let result = AccountOwnerRef::try_from("ValidRef");
        assert_eq!(result, Ok(AccountOwnerRef("ValidRef".to_string())));
        assert_eq!(result.unwrap().to_string(), "ValidRef");
    }
}
