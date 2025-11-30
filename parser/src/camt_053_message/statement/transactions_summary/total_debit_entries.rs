use rust_decimal::Decimal;

#[derive(Debug, PartialEq)]
pub(super) struct TotalDebitEntries {
    number: Option<usize>,
    sum: Option<Decimal>,
}
