use crate::camt_053_message::group_header::*;

mod group_header;

pub struct Camt053Message {
    group_header: GroupHeader,
}
