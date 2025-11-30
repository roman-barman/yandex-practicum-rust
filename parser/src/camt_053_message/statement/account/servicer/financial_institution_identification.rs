#[derive(Debug, PartialEq)]
pub(super) struct FinancialInstitutionIdentification {
    bic: Option<Bic>,
}

#[derive(Debug, PartialEq)]
pub(super) struct Bic(String);
