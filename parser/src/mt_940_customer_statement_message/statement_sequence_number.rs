use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::Write;

const SEQUENCE_NUMBER_MAX_LENGTH: usize = 5;
const STATEMENT_NUMBER_MAX_LENGTH: usize = 5;

#[derive(Debug, PartialEq)]
pub(super) struct StatementSequenceNumber {
    statement_number: u16,
    sequence_number: Option<u16>,
}

impl StatementSequenceNumber {
    pub(super) fn write_to<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        if self.sequence_number.is_none() {
            writer.write_all(self.statement_number.to_string().as_bytes())
        } else {
            writer.write_all(
                format!(
                    "{}/{}",
                    self.statement_number,
                    self.sequence_number.unwrap()
                )
                .as_bytes(),
            )
        }
    }
}

impl TryFrom<&str> for StatementSequenceNumber {
    type Error = StatementSequenceNumberParseError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Err(StatementSequenceNumberParseError::Empty);
        }

        let split = value.split('/').collect::<Vec<&str>>();
        if split.len() > 2 {
            return Err(StatementSequenceNumberParseError::InvalidStatementSequenceNumberFormat);
        }

        let statement_number = split[0];
        if statement_number.len() > STATEMENT_NUMBER_MAX_LENGTH {
            return Err(StatementSequenceNumberParseError::StatementNumberTooLong);
        }
        let statement_number = statement_number
            .parse::<u16>()
            .map_err(|_| StatementSequenceNumberParseError::InvalidStatementNumberFormat)?;

        if split.len() == 1 {
            return Ok(Self {
                statement_number,
                sequence_number: None,
            });
        }

        let sequence_number = split[1];
        if sequence_number.len() > SEQUENCE_NUMBER_MAX_LENGTH {
            return Err(StatementSequenceNumberParseError::SequenceNumberTooLong);
        }
        let sequence_number = sequence_number
            .parse::<u16>()
            .map_err(|_| StatementSequenceNumberParseError::InvalidSequenceNumberFormat)?;
        Ok(Self {
            statement_number,
            sequence_number: Some(sequence_number),
        })
    }
}

impl Display for StatementSequenceNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.sequence_number.is_none() {
            write!(f, "{}", self.statement_number)
        } else {
            write!(
                f,
                "{}/{}",
                self.statement_number,
                self.sequence_number.unwrap()
            )
        }
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum StatementSequenceNumberParseError {
    Empty,
    InvalidStatementSequenceNumberFormat,
    StatementNumberTooLong,
    InvalidStatementNumberFormat,
    SequenceNumberTooLong,
    InvalidSequenceNumberFormat,
}

impl Display for StatementSequenceNumberParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            StatementSequenceNumberParseError::Empty => {
                write!(f, "Empty statement sequence number")
            }
            StatementSequenceNumberParseError::InvalidStatementSequenceNumberFormat => {
                write!(f, "Invalid statement sequence number format")
            }
            StatementSequenceNumberParseError::StatementNumberTooLong => write!(
                f,
                "Statement number cannot be longer than {} characters",
                STATEMENT_NUMBER_MAX_LENGTH
            ),
            StatementSequenceNumberParseError::InvalidStatementNumberFormat => {
                write!(f, "Invalid statement number format")
            }
            StatementSequenceNumberParseError::SequenceNumberTooLong => write!(
                f,
                "Sequence number cannot be longer than {} characters",
                SEQUENCE_NUMBER_MAX_LENGTH
            ),
            StatementSequenceNumberParseError::InvalidSequenceNumberFormat => {
                write!(f, "Invalid sequence number format")
            }
        }
    }
}

impl Error for StatementSequenceNumberParseError {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_statement_sequence_number_write_to() {
        let mut buffer = Cursor::new(Vec::new());
        StatementSequenceNumber {
            statement_number: 12345,
            sequence_number: Some(2),
        }
        .write_to(&mut buffer)
        .unwrap();
        assert_eq!(buffer.get_ref(), b"12345/2");

        let mut buffer = Cursor::new(Vec::new());
        StatementSequenceNumber {
            statement_number: 12345,
            sequence_number: None,
        }
        .write_to(&mut buffer)
        .unwrap();
        assert_eq!(buffer.get_ref(), b"12345");
    }

    #[test]
    fn test_empty_statement_sequence_number() {
        let result = StatementSequenceNumber::try_from("");
        assert_eq!(result, Err(StatementSequenceNumberParseError::Empty));
        assert_eq!(
            result.unwrap_err().to_string(),
            "Empty statement sequence number"
        );
    }

    #[test]
    fn test_invalid_statement_sequence_number_format() {
        let result = StatementSequenceNumber::try_from("1/2/3");
        assert_eq!(
            result,
            Err(StatementSequenceNumberParseError::InvalidStatementSequenceNumberFormat)
        );
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid statement sequence number format"
        );
    }

    #[test]
    fn test_statement_number_too_long() {
        let result = StatementSequenceNumber::try_from("123456/2");
        assert_eq!(
            result,
            Err(StatementSequenceNumberParseError::StatementNumberTooLong)
        );
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Statement number cannot be longer than {} characters",
                STATEMENT_NUMBER_MAX_LENGTH
            )
        );
    }

    #[test]
    fn test_invalid_statement_number_format() {
        let result = StatementSequenceNumber::try_from("123K/2");
        assert_eq!(
            result,
            Err(StatementSequenceNumberParseError::InvalidStatementNumberFormat)
        );
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid statement number format"
        );
    }

    #[test]
    fn test_sequence_number_too_long() {
        let result = StatementSequenceNumber::try_from("12345/123456");
        assert_eq!(
            result,
            Err(StatementSequenceNumberParseError::SequenceNumberTooLong)
        );
        assert_eq!(
            result.unwrap_err().to_string(),
            format!(
                "Sequence number cannot be longer than {} characters",
                SEQUENCE_NUMBER_MAX_LENGTH
            )
        );
    }

    #[test]
    fn test_invalid_sequence_number_format() {
        let result = StatementSequenceNumber::try_from("12345/2S");
        assert_eq!(
            result,
            Err(StatementSequenceNumberParseError::InvalidSequenceNumberFormat)
        );
        assert_eq!(
            result.unwrap_err().to_string(),
            "Invalid sequence number format"
        );
    }

    #[test]
    fn test_valid_statement_sequence_number() {
        let result = StatementSequenceNumber::try_from("12345/2");
        assert_eq!(
            result,
            Ok(StatementSequenceNumber {
                statement_number: 12345,
                sequence_number: Some(2)
            })
        );
        assert_eq!(result.unwrap().to_string(), "12345/2");

        let result = StatementSequenceNumber::try_from("12345");
        assert_eq!(
            result,
            Ok(StatementSequenceNumber {
                statement_number: 12345,
                sequence_number: None
            })
        );
        assert_eq!(result.unwrap().to_string(), "12345");

        let result = StatementSequenceNumber::try_from("001/002");
        assert_eq!(
            result,
            Ok(StatementSequenceNumber {
                statement_number: 1,
                sequence_number: Some(2)
            })
        );
        assert_eq!(result.unwrap().to_string(), "1/2");
    }
}
