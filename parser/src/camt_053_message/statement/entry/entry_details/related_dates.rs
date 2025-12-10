use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct RelatedDates {
    #[serde(rename = "AccptncDtTm")]
    acceptance_date_time: Option<NaiveDateTime>,
}

impl Display for RelatedDates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(acceptance_date_time) = &self.acceptance_date_time {
            writeln!(f, "- Acceptance date time: {}", acceptance_date_time)?;
        }
        Ok(())
    }
}

impl RelatedDates {
    pub(crate) fn new(acceptance_date_time: Option<NaiveDateTime>) -> Self {
        Self {
            acceptance_date_time,
        }
    }
}
