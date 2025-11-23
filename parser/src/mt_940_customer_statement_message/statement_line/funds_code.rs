use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub(super) struct FundsCode(char);

impl TryFrom<&char> for FundsCode {
    type Error = FundsCodeParseError;
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        if !(value.is_ascii_alphabetic() && value.is_ascii_uppercase()) {
            Err(FundsCodeParseError::InvalidFormat)
        } else {
            Ok(Self(*value))
        }
    }
}

impl Display for FundsCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum FundsCodeParseError {
    InvalidFormat,
}

impl Display for FundsCodeParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid funds code format")
    }
}

impl Error for FundsCodeParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_funds_code() {
        let result = FundsCode::try_from(&'1');
        assert_eq!(result, Err(FundsCodeParseError::InvalidFormat));
        assert_eq!(result.unwrap_err().to_string(), "Invalid funds code format");

        let result = FundsCode::try_from(&'a');
        assert_eq!(result, Err(FundsCodeParseError::InvalidFormat));
        assert_eq!(result.unwrap_err().to_string(), "Invalid funds code format");
    }

    #[test]
    fn test_valid_funds_code() {
        let result = FundsCode::try_from(&'A');
        assert_eq!(result, Ok(FundsCode('A')));
        assert_eq!(result.unwrap().to_string(), "A");
    }
}
