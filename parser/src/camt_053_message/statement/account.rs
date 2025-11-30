use crate::camt_053_message::statement::account::account_identification::*;
use crate::camt_053_message::statement::account::name::*;
use crate::camt_053_message::statement::account::owner::*;
use crate::camt_053_message::statement::account::servicer::*;
use crate::camt_053_message::statement::currency::*;

mod account_identification;
mod name;
mod owner;
mod postal_address;
mod servicer;

#[derive(Debug, PartialEq)]
pub(super) struct Account {
    identification: AccountIdentification,
    currency: Currency,
    name: Option<Name>,
    owner: Option<Owner>,
    servicer: Option<Servicer>,
}
