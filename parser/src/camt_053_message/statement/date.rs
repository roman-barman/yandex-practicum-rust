use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, PartialEq)]
pub(super) enum Date {
    Date(NaiveDate),
    DateTime(DateTime<Utc>),
}
