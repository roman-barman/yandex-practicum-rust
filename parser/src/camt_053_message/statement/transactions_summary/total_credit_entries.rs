use rust_decimal::Decimal;

#[derive(Debug, PartialEq)]
pub(super) struct TotalCreditEntries {
    number: Option<usize>,
    sum: Option<Decimal>,
}
