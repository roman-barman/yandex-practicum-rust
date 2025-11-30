#[derive(Debug, PartialEq)]
pub(super) struct BankTransactionCode {
    domain: Domain,
    proprietary: Proprietary,
}

#[derive(Debug, PartialEq)]
pub(super) struct Domain {
    code: String,
    family: Family,
}

#[derive(Debug, PartialEq)]
pub(super) struct Family {
    code: String,
    sub_family_code: String,
}

#[derive(Debug, PartialEq)]
pub(super) struct Proprietary {
    code: String,
    issuer: String,
}
