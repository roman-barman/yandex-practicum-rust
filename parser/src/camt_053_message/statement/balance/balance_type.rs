#[derive(Debug, PartialEq)]
pub(super) struct BalanceType {
    code_or_proprietary: CodeOrProprietary,
}

#[derive(Debug, PartialEq)]
pub(super) enum CodeOrProprietary {
    Code(String),
    Proprietary(String),
}
