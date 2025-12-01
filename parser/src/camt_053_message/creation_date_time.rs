use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct CreationDateTime(NaiveDateTime);

impl CreationDateTime {
    pub(super) fn new(creation_date_time: NaiveDateTime) -> Self {
        CreationDateTime(creation_date_time)
    }
}
