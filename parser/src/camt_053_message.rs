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
    use rust_decimal::Decimal;

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
            <CreDtTm>2023-04-20T23:24:33</CreDtTm>
            <FrToDt>
                <FrDtTm>2023-04-20T00:00:00</FrDtTm>
                <ToDtTm>2023-04-20T23:59:59</ToDtTm>
            </FrToDt>
            <Acct>
                <Id>
                    <IBAN>DK8030000001234567</IBAN>
                </Id>
                <Ccy>DKK</Ccy>
                <Nm>Danske Corporate</Nm>
                <Ownr>
                    <Nm>Account owner</Nm>
                    <PstlAdr>
                        <StrtNm>Streetname</StrtNm>
                        <BldgNb>20</BldgNb>
                        <PstCd>1234</PstCd>
                        <TwnNm>Townname</TwnNm>
                        <Ctry>DK</Ctry>
                    </PstlAdr>
                    <Id>
                        <OrgId>
                            <Othr>
                                <Id>0012345678</Id>
                                <SchmeNm>
                                    <Cd>CUST</Cd>
                                </SchmeNm>
                            </Othr>
                        </OrgId>
                    </Id>
                </Ownr>
                <Svcr>
                    <FinInstnId>
                        <BIC>DABADKKK</BIC>
                    </FinInstnId>
                </Svcr>
            </Acct>
            <Bal>
                <Tp>
                    <CdOrPrtry>
                        <Cd>OPBD</Cd>
                    </CdOrPrtry>
                </Tp>
                <Amt Ccy=\"DKK\">12345.67</Amt>
                <CdtDbtInd>DBIT</CdtDbtInd>
                <Dt>
                    <Dt>2023-04-20</Dt>
                </Dt>
            </Bal>
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
                Some(sequence_number::SequenceNumber::new(1)),
                CreationDateTime::new(
                    NaiveDate::from_ymd_opt(2023, 4, 20)
                        .unwrap()
                        .and_hms_opt(23, 24, 33)
                        .unwrap()
                ),
                Some(from_to_date::FromToDate::new(
                    NaiveDate::from_ymd_opt(2023, 4, 20)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                    NaiveDate::from_ymd_opt(2023, 4, 20)
                        .unwrap()
                        .and_hms_opt(23, 59, 59)
                        .unwrap()
                )),
                account::Account::new(
                    account::account_identification::AccountIdentification::IBAN(
                        Identification::new("DK8030000001234567".to_string())
                    ),
                    currency::Currency::new("DKK".to_string()),
                    Some(account::name::Name::new("Danske Corporate".to_string())),
                    Some(account::owner::Owner::new(
                        Some(account::name::Name::new("Account owner".to_string())),
                        Some(account::postal_address::PostalAddress::new(
                            Some("Streetname".to_string()),
                            Some("20".to_string()),
                            Some("1234".to_string()),
                            Some("Townname".to_string()),
                            Some("DK".to_string()))),
                        Some(account::owner::owner_identification::OwnerIdentification::Organization(
                            account::owner::owner_identification::OrganizationIdentification::Other{
                                id: Identification::new("0012345678".to_string()),
                                scheme_name: Some(account::owner::owner_identification::SchemeName::Code("CUST".to_string()))
                            })))
                    ),
                    Some(account::servicer::Servicer::new(
                        account::servicer::financial_institution_identification::FinancialInstitutionIdentification::new(
                            Some("DABADKKK".to_string()))))
                ),
                vec![balance::Balance::new(
                    balance::balance_type::BalanceType::new(balance::balance_type::CodeOrProprietary::Code("OPBD".to_string())),
                    amount::Amount::new(currency::Currency::new("DKK".to_string()), Decimal::new(1234567, 2)),
                    credit_debit_identification::CreditDebitIdentification::new("DBIT".to_string()),
                    date::Date::Date(NaiveDate::from_ymd_opt(2023, 4, 20).unwrap())
                )]
            )
        );
    }
}
