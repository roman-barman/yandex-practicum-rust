use std::fmt::Display;
use thiserror::Error;

const SUPPLEMENTARY_DETAILS_MAX_LENGTH: usize = 34;

#[derive(Debug, PartialEq)]
pub(crate) struct SupplementaryDetails(String);

impl SupplementaryDetails {
    pub(super) fn write_to<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.0.as_bytes())?;
        Ok(())
    }
}

impl TryFrom<&str> for SupplementaryDetails {
    type Error = SupplementaryDetailsParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(SupplementaryDetailsParseError::Empty);
        }
        if value.len() > SUPPLEMENTARY_DETAILS_MAX_LENGTH {
            return Err(SupplementaryDetailsParseError::TooLong);
        }
        Ok(Self(value.to_string()))
    }
}

impl Display for SupplementaryDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Error)]
pub(crate) enum SupplementaryDetailsParseError {
    #[error("Supplementary details cannot be empty")]
    Empty,
    #[error(
        "Supplementary details cannot be longer than {} characters",
        SUPPLEMENTARY_DETAILS_MAX_LENGTH
    )]
    TooLong,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_supplementary_details_write_to() {
        let mut buffer = Cursor::new(Vec::new());
        SupplementaryDetails("ValidDetails".to_string())
            .write_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer.get_ref(), b"ValidDetails");
    }

    #[test]
    fn test_empty_supplementary_details() {
        let result = SupplementaryDetails::try_from("");
        assert_eq!(result, Err(SupplementaryDetailsParseError::Empty));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Supplementary details cannot be empty"
        );
    }

    #[test]
    fn test_long_supplementary_details() {
        let result = SupplementaryDetails::try_from(
            "1".repeat(SUPPLEMENTARY_DETAILS_MAX_LENGTH + 1).as_str(),
        );
        assert_eq!(result, Err(SupplementaryDetailsParseError::TooLong));
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Supplementary details cannot be longer than {} characters",
                SUPPLEMENTARY_DETAILS_MAX_LENGTH
            )
        );
    }

    #[test]
    fn test_valid_supplementary_details() {
        let result = SupplementaryDetails::try_from("ValidDetails");
        assert_eq!(result, Ok(SupplementaryDetails("ValidDetails".to_string())));
        assert_eq!(result.unwrap().to_string(), "ValidDetails");
    }
}
