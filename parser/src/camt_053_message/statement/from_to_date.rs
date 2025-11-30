use chrono::{DateTime, Utc};

#[derive(Debug, PartialEq)]
pub(super) struct FromToDate {
    from: DateTime<Utc>,
    to: DateTime<Utc>,
}
