use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub(super) struct FundsCode(char);

impl FundsCode {
    pub(super) fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[self.0 as u8])
    }
}

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

#[derive(Debug, PartialEq, Error)]
pub(crate) enum FundsCodeParseError {
    #[error("Invalid funds code format")]
    InvalidFormat,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_funds_code_write_to() {
        let mut buffer = Cursor::new(Vec::new());
        FundsCode('A').write_to(&mut buffer).unwrap();
        assert_eq!(buffer.get_ref(), b"A");
    }

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
