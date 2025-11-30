use crate::camt_053_message::statement::account::servicer::financial_institution_identification::*;

mod financial_institution_identification;

#[derive(Debug, PartialEq)]
pub(super) struct Servicer {
    identification: FinancialInstitutionIdentification,
}
