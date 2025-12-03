use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum Date {
    #[serde(rename = "Dt")]
    Date(NaiveDate),
    #[serde(rename = "DtTm")]
    DateTime(NaiveDateTime),
}
