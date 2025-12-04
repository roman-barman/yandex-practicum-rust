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
            <TxsSummry>
                <TtlNtries>
                    <NbOfNtries>6</NbOfNtries>
                    <TtlNetNtryAmt>343174.12</TtlNetNtryAmt>
                    <CdtDbtInd>DBIT</CdtDbtInd>
                </TtlNtries>
                <TtlCdtNtries>
                    <NbOfNtries>4</NbOfNtries>
                    <Sum>3163.89</Sum>
                </TtlCdtNtries>
                <TtlDbtNtries>
                    <NbOfNtries>2</NbOfNtries>
                    <Sum>346338.01</Sum>
                </TtlDbtNtries>
            </TxsSummry>
            <Ntry>
                <NtryRef>1</NtryRef>
                <Amt Ccy=\"DKK\">591.15</Amt>
                <CdtDbtInd>CRDT</CdtDbtInd>
                <Sts>BOOK</Sts>
                 <BookgDt>
                    <Dt>2023-04-20</Dt>
                </BookgDt>
                <ValDt>
                    <Dt>2023-04-20</Dt>
                </ValDt>
                <AcctSvcrRef>012X123456789012</AcctSvcrRef>
                <BkTxCd>
                    <Domn>
                        <Cd>PMNT</Cd>
                        <Fmly>
                            <Cd>RCDT</Cd>
                            <SubFmlyCd>XBCT</SubFmlyCd>
                        </Fmly>
                    </Domn>
                    <Prtry>
                        <Cd>BETAL. 3825-0123456789</Cd>
                        <Issr>DBA</Issr>
                    </Prtry>
                </BkTxCd>
                <AddtlInfInd>
                    <MsgNmId>O1XXXXXXX67X1X</MsgNmId>
                </AddtlInfInd>
            </Ntry>
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
                )],
                Some(transactions_summary::TransactionsSummary::new(
                    Some(transactions_summary::total_entries::TotalEntries::new(
                        Some(6),
                        Some(Decimal::new(34317412, 2)),
                        Some(credit_debit_identification::CreditDebitIdentification::new("DBIT".to_string())))),
                    Some(transactions_summary::total_credit_entries::TotalCreditEntries::new(Some(4), Some(Decimal::new(316389, 2)))),
                    Some(transactions_summary::total_debit_entries::TotalDebitEntries::new(Some(2), Some(Decimal::new(34633801, 2))))
                )),
                Some(vec![entry::Entry::new(
                    Some(entry::entry_reference::EntryReference::new("1".to_string())),
                    amount::Amount::new(currency::Currency::new("DKK".to_string()), Decimal::new(59115, 2)),
                    credit_debit_identification::CreditDebitIdentification::new("CRDT".to_string()),
                    entry::status::Status::new("BOOK".to_string()),
                    Some(date::Date::Date(NaiveDate::from_ymd_opt(2023, 4, 20).unwrap())),
                    Some(date::Date::Date(NaiveDate::from_ymd_opt(2023, 4, 20).unwrap())),
                    Some(entry::account_servicer_reference::AccountServicerReference::new("012X123456789012".to_string())),
                    entry::bank_transaction_code::BankTransactionCode::new(
                        Some(entry::bank_transaction_code::Domain::new(
                            "PMNT".to_string(),
                            entry::bank_transaction_code::Family::new("RCDT".to_string(), "XBCT".to_string())
                        )),
                        Some(entry::bank_transaction_code::Proprietary::new(
                            "BETAL. 3825-0123456789".to_string(),
                            Some("DBA".to_string())
                        ))
                    ),
                    Some(entry::additional_information_indicator::AdditionalInformationIndicator::new(
                        Some(Identification::new("O1XXXXXXX67X1X".to_string()))
                    ))
                )])
            )
        );
    }
}
