use crate::camt_053_message::group_header::creation_date_time::CreationDateTime;
use crate::camt_053_message::group_header::message_identification::*;

mod creation_date_time;
mod message_identification;

pub(super) struct GroupHeader {
    message_identification: MessageIdentification,
    creation_date_time: CreationDateTime,
}
