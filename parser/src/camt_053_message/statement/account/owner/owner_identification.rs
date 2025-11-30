use crate::camt_053_message::identification::Identification;

#[derive(Debug, PartialEq)]
pub(super) enum OwnerIdentification {
    Organization(OrganizationIdentification),
    Private,
}

#[derive(Debug, PartialEq)]
pub(super) enum OrganizationIdentification {
    BicOrBei(String),
    Other(Identification, Option<SchemeName>),
}

#[derive(Debug, PartialEq)]
pub(super) enum SchemeName {
    Code(String),
    Proprietary(String),
}
