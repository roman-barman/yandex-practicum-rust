use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct FromToDate {
    #[serde(rename = "FrDtTm")]
    from: NaiveDateTime,
    #[serde(rename = "ToDtTm")]
    to: NaiveDateTime,
}

impl FromToDate {
    pub fn new(from: NaiveDateTime, to: NaiveDateTime) -> Self {
        Self { from, to }
    }
}
