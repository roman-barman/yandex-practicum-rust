use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct RelatedDates {
    #[serde(rename = "AccptncDtTm")]
    acceptance_date_time: Option<NaiveDateTime>,
}

impl RelatedDates {
    pub(crate) fn new(acceptance_date_time: Option<NaiveDateTime>) -> Self {
        Self {
            acceptance_date_time,
        }
    }
}
