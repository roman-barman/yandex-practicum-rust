use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct FromToDate {
    #[serde(rename = "FrDtTm")]
    from: NaiveDateTime,
    #[serde(rename = "ToDtTm")]
    to: NaiveDateTime,
}

impl Display for FromToDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} to {}", self.from, self.to)
    }
}

impl FromToDate {
    pub fn new(from: NaiveDateTime, to: NaiveDateTime) -> Self {
        Self { from, to }
    }
}
