use std::fmt::Display;
use std::io::Write;
use thiserror::Error;

const RELATED_REFERENCE_MAX_LENGTH: usize = 16;

#[derive(Debug, PartialEq)]
pub(super) struct RelatedReference(String);

impl RelatedReference {
    pub(super) fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(self.0.as_bytes())
    }
}

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

        Ok(Self(value.to_string()))
    }
}

impl Display for RelatedReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Error)]
pub(super) enum RelatedReferenceParseError {
    #[error("Related reference cannot be empty")]
    Empty,
    #[error(
        "Related reference cannot be longer than {} characters",
        RELATED_REFERENCE_MAX_LENGTH
    )]
    TooLong,
    #[error("Related reference has invalid format")]
    InvalidFormat,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_related_reference_write_to() {
        let mut buffer = Cursor::new(Vec::new());
        RelatedReference("1234567890".to_string())
            .write_to(&mut buffer)
            .unwrap();
        assert_eq!(buffer.get_ref(), b"1234567890");
    }

    #[test]
    fn test_empty_related_reference() {
        let result = RelatedReference::try_from("");
        assert_eq!(result, Err(RelatedReferenceParseError::Empty));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Related reference cannot be empty"
        );
    }

    #[test]
    fn test_related_reference_too_long() {
        let result =
            RelatedReference::try_from("1".repeat(RELATED_REFERENCE_MAX_LENGTH + 1).as_str());
        assert_eq!(result, Err(RelatedReferenceParseError::TooLong));
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Related reference cannot be longer than {} characters",
                RELATED_REFERENCE_MAX_LENGTH
            )
        );
    }

    #[test]
    fn test_invalid_related_reference() {
        let result = RelatedReference::try_from("/12345678901");
        assert_eq!(result, Err(RelatedReferenceParseError::InvalidFormat));

        let result = RelatedReference::try_from("12345678901/");
        assert_eq!(result, Err(RelatedReferenceParseError::InvalidFormat));

        let result = RelatedReference::try_from("12//345678901");
        assert_eq!(result, Err(RelatedReferenceParseError::InvalidFormat));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Related reference has invalid format"
        );
    }

    #[test]
    fn test_valid_related_reference() {
        let result = RelatedReference::try_from("1234567890");
        assert_eq!(result, Ok(RelatedReference("1234567890".to_string())));
        assert_eq!(result.unwrap().to_string(), "1234567890");
    }
}
