use crate::camt_053_message::identification::*;

#[derive(Debug, PartialEq)]
pub(super) enum AccountIdentification {
    IBAN(Identification),
    Other,
}
