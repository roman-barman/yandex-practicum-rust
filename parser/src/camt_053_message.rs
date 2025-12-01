use crate::camt_053_message::group_header::*;
use crate::camt_053_message::statement::*;
use serde::{Deserialize, Serialize};
use std::io::Read;

mod creation_date_time;
mod group_header;
mod identification;
mod statement;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Camt053Message {
    #[serde(rename = "GrpHdr")]
    group_header: GroupHeader,
    #[serde(rename = "Stmt")]
    statements: Vec<Statement>,
}

impl Camt053Message {
    pub fn read_from<T: Read>(reader: T) -> Result<Self, String> {
        let result: Document = serde_xml_rs::from_reader(reader).unwrap();
        Ok(result.camt053message)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Document {
    #[serde(rename = "BkToCstmrStmt")]
    camt053message: Camt053Message,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::camt_053_message::creation_date_time::*;
    use crate::camt_053_message::identification::*;
    use crate::camt_053_message::statement::*;
    use chrono::NaiveDate;

    const DATA: &str = "
<Document xmlns=\"urn:iso:std:iso:20022:tech:xsd:camt.053.001.02\"
    xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\"
    xsi:schemaLocation=\"urn:iso:std:iso:20022:tech:xsd:camt.053.001.02 camt.053.001.02.xsd\">
    <BkToCstmrStmt>
        <GrpHdr>
            <MsgId>XXX24Y4XXX1Y000000001</MsgId>
            <CreDtTm>2023-04-20T23:24:31</CreDtTm>
        </GrpHdr>
        <Stmt>
            <Id>XXX24Y4XXX1Y000000001</Id>
            <ElctrncSeqNb>1</ElctrncSeqNb>
            <LglSeqNb>1</LglSeqNb>
        </Stmt>
    </BkToCstmrStmt>
</Document>";

    #[test]
    fn test_read_from() {
        let result = Camt053Message::read_from(DATA.as_bytes());
        assert!(result.is_ok());

        let message = result.unwrap();
        assert_eq!(
            message.group_header,
            GroupHeader::new(
                Identification::new("XXX24Y4XXX1Y000000001".to_string()),
                CreationDateTime::new(
                    NaiveDate::from_ymd_opt(2023, 4, 20)
                        .unwrap()
                        .and_hms_opt(23, 24, 31)
                        .unwrap()
                )
            )
        );
        assert_eq!(message.statements.len(), 1);
        let statement = &message.statements[0];
        assert_eq!(
            *statement,
            Statement::new(
                Identification::new("XXX24Y4XXX1Y000000001".to_string()),
                sequence_number::SequenceNumber::new(1),
                Some(sequence_number::SequenceNumber::new(1))
            )
        );
    }
}
