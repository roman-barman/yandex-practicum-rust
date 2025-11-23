use std::error::Error;
use std::fmt::Display;

const SUPPLEMENTARY_DETAILS_MAX_LENGTH: usize = 34;

#[derive(Debug, PartialEq)]
pub(super) struct SupplementaryDetails(String);

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

#[derive(Debug, PartialEq)]
pub(super) enum SupplementaryDetailsParseError {
    Empty,
    TooLong,
}
impl Display for SupplementaryDetailsParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SupplementaryDetailsParseError::Empty => {
                write!(f, "Supplementary details cannot be empty")
            }
            SupplementaryDetailsParseError::TooLong => write!(
                f,
                "Supplementary details cannot be longer than {} characters",
                SUPPLEMENTARY_DETAILS_MAX_LENGTH
            ),
        }
    }
}

impl Error for SupplementaryDetailsParseError {}

#[cfg(test)]
mod tests {
    use super::*;

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
