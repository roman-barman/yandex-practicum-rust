use std::fmt::Display;
use thiserror::Error;

const BANK_REF_MAX_LENGTH: usize = 16;

#[derive(Debug, PartialEq)]
pub(super) struct BankRef(String);

impl BankRef {
    pub(super) fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.0.as_bytes())?;
        Ok(())
    }
}

impl TryFrom<&str> for BankRef {
    type Error = BankRefParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(BankRefParseError::Empty);
        }
        if value.len() > BANK_REF_MAX_LENGTH {
            return Err(BankRefParseError::TooLong);
        }
        Ok(Self(value.to_string()))
    }
}

impl Display for BankRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Error)]
pub(crate) enum BankRefParseError {
    #[error("Bank ref cannot be empty")]
    Empty,
    #[error("Bank ref cannot be longer than {} characters", BANK_REF_MAX_LENGTH)]
    TooLong,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bank_ref_write_to() {
        let mut buffer = Vec::new();
        BankRef("ValidRef".to_string())
            .write_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer, b"ValidRef");
    }

    #[test]
    fn test_empty_bank_ref() {
        let result = BankRef::try_from("");
        assert_eq!(result, Err(BankRefParseError::Empty));
        assert_eq!(result.unwrap_err().to_string(), "Bank ref cannot be empty");
    }

    #[test]
    fn test_long_bank_ref() {
        let result = BankRef::try_from("1".repeat(BANK_REF_MAX_LENGTH + 1).as_str());
        assert_eq!(result, Err(BankRefParseError::TooLong));
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Bank ref cannot be longer than {} characters",
                BANK_REF_MAX_LENGTH
            )
        );
    }

    #[test]
    fn test_valid_bank_ref() {
        let result = BankRef::try_from("ValidRef");
        assert_eq!(result, Ok(BankRef("ValidRef".to_string())));
        assert_eq!(result.unwrap().to_string(), "ValidRef");
    }
}
