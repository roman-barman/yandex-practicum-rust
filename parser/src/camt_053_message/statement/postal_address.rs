use serde::{Deserialize, Serialize};

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
