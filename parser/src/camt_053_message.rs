use crate::camt_053_message::group_header::*;
use crate::camt_053_message::statement::*;

mod creation_date_time;
mod group_header;
mod identification;
mod statement;

pub struct Camt053Message {
    group_header: GroupHeader,
    statements: Vec<Statement>,
}
