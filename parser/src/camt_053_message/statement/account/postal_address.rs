#[derive(Debug, PartialEq)]
pub(super) struct PostalAddress {
    street: Option<String>,
    building_number: Option<String>,
    post_code: Option<String>,
    town_name: Option<String>,
    country: Option<String>,
}
