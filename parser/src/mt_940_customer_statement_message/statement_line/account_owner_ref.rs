use std::error::Error;
use std::fmt::Display;

const ACCOUNT_OWNER_REF_MAX_LENGTH: usize = 16;

#[derive(Debug, PartialEq)]
pub(super) struct AccountOwnerRef(String);

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

#[derive(Debug, PartialEq)]
pub(super) enum AccountOwnerRefParseError {
    Empty,
    TooLong,
}

impl Display for AccountOwnerRefParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountOwnerRefParseError::Empty => write!(f, "Account owner ref cannot be empty"),
            AccountOwnerRefParseError::TooLong => write!(
                f,
                "Account owner ref cannot be longer than {} characters",
                ACCOUNT_OWNER_REF_MAX_LENGTH
            ),
        }
    }
}

impl Error for AccountOwnerRefParseError {}

#[cfg(test)]
mod tests {
    use super::*;

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
