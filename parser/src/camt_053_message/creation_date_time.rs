use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) struct CreationDateTime(NaiveDateTime);

impl CreationDateTime {
    pub(super) fn now() -> Self {
        CreationDateTime(Local::now().naive_local())
    }
}

impl Display for CreationDateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
impl CreationDateTime {
    pub(super) fn new(creation_date_time: NaiveDateTime) -> Self {
        CreationDateTime(creation_date_time)
    }
}
