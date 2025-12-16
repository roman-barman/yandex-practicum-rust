use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) enum Date {
    #[serde(rename = "Dt")]
    Date(NaiveDate),
    #[serde(rename = "DtTm")]
    DateTime(NaiveDateTime),
}

impl From<&crate::mt_940_customer_statement_message::date::Date> for Date {
    fn from(value: &crate::mt_940_customer_statement_message::date::Date) -> Self {
        Date::Date(value.as_ref().clone())
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Date::Date(date) => write!(f, "{}", date),
            Date::DateTime(datetime) => write!(f, "{}", datetime),
        }
    }
}
