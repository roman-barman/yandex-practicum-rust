use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(crate) struct PostalAddress {
    #[serde(rename = "StrtNm")]
    street: Option<String>,
    #[serde(rename = "BldgNb")]
    building_number: Option<String>,
    #[serde(rename = "PstCd")]
    post_code: Option<String>,
    #[serde(rename = "TwnNm")]
    town_name: Option<String>,
    #[serde(rename = "Ctry")]
    country: Option<String>,
    #[serde(rename = "AdrLine")]
    address_line: Option<Vec<String>>,
}

impl Display for PostalAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(street) = &self.street {
            writeln!(f, "- Street: {}", street)?;
        }
        if let Some(building_number) = &self.building_number {
            writeln!(f, "- Building number: {}", building_number)?;
        }
        if let Some(post_code) = &self.post_code {
            writeln!(f, "- Post code: {}", post_code)?;
        }
        if let Some(town_name) = &self.town_name {
            writeln!(f, "- Town name: {}", town_name)?;
        }
        if let Some(country) = &self.country {
            writeln!(f, "- Country: {}", country)?;
        }
        if let Some(address_line) = &self.address_line {
            for line in address_line {
                writeln!(f, "- Address line: {}", line)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
impl PostalAddress {
    pub(crate) fn new(
        street: Option<String>,
        building_number: Option<String>,
        post_code: Option<String>,
        town_name: Option<String>,
        country: Option<String>,
        address_line: Option<Vec<String>>,
    ) -> Self {
        Self {
            street,
            building_number,
            post_code,
            town_name,
            country,
            address_line,
        }
    }
}
