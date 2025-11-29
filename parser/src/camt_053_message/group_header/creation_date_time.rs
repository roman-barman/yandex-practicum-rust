use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq)]
pub(super) struct CreationDateTime(DateTime<Utc>);
