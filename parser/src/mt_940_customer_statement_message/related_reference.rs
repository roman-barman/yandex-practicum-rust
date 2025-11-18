use std::error::Error;
use std::fmt::Display;

const RELATED_REFERENCE_MAX_LENGTH: usize = 16;

#[derive(Debug, PartialEq)]
pub(crate) struct RelatedReference(String);

impl TryFrom<&str> for RelatedReference {
    type Error = RelatedReferenceParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(RelatedReferenceParseError::Empty);
        }

        if value.len() > RELATED_REFERENCE_MAX_LENGTH {
            return Err(RelatedReferenceParseError::TooLong);
        }

        if value.starts_with("/") || value.ends_with("/") || value.contains("//") {
            return Err(RelatedReferenceParseError::InvalidFormat);
        }

        Ok(RelatedReference(value.to_string()))
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum RelatedReferenceParseError {
    Empty,
    TooLong,
    InvalidFormat,
}

impl Display for RelatedReferenceParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RelatedReferenceParseError::Empty => write!(f, "Related reference cannot be empty"),
            RelatedReferenceParseError::TooLong => write!(
                f,
                "Related reference exceeds {} character length",
                RELATED_REFERENCE_MAX_LENGTH
            ),
            RelatedReferenceParseError::InvalidFormat => {
                write!(f, "Related reference has invalid format")
            }
        }
    }
}

impl Error for RelatedReferenceParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_related_reference() {
        let result = RelatedReference::try_from("");
        assert_eq!(result, Err(RelatedReferenceParseError::Empty));
    }

    #[test]
    fn test_related_reference_too_long() {
        let result =
            RelatedReference::try_from("1".repeat(RELATED_REFERENCE_MAX_LENGTH + 1).as_str());
        assert_eq!(result, Err(RelatedReferenceParseError::TooLong));
    }

    #[test]
    fn test_invalid_related_reference() {
        let result = RelatedReference::try_from("/12345678901");
        assert_eq!(result, Err(RelatedReferenceParseError::InvalidFormat));

        let result = RelatedReference::try_from("12345678901/");
        assert_eq!(result, Err(RelatedReferenceParseError::InvalidFormat));

        let result = RelatedReference::try_from("12//345678901");
        assert_eq!(result, Err(RelatedReferenceParseError::InvalidFormat));
    }

    #[test]
    fn test_valid_related_reference() {
        let result = RelatedReference::try_from("1234567890");
        assert_eq!(result, Ok(RelatedReference("1234567890".to_string())));
    }
}
