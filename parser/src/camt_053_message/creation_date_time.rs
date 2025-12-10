use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct CreationDateTime(NaiveDateTime);

impl Display for CreationDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CreationDateTime {
    pub(super) fn new(creation_date_time: NaiveDateTime) -> Self {
        CreationDateTime(creation_date_time)
    }
}
