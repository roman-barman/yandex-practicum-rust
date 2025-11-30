use crate::camt_053_message::creation_date_time::*;
use crate::camt_053_message::identification::*;

pub(super) struct GroupHeader {
    message_identification: Identification,
    creation_date_time: CreationDateTime,
}
