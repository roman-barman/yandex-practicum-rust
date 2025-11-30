use crate::camt_053_message::statement::account::name::*;
use crate::camt_053_message::statement::account::owner::owner_identification::*;
use crate::camt_053_message::statement::account::postal_address::*;

mod owner_identification;

#[derive(Debug, PartialEq)]
pub(super) struct Owner {
    name: Option<Name>,
    postal_address: Option<PostalAddress>,
    identification: Option<OwnerIdentification>,
}
