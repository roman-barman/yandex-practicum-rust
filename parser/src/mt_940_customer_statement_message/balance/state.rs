use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub(crate) enum State {
    Final,
    Intermediate,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Final => write!(f, "Final"),
            State::Intermediate => write!(f, "Intermediate"),
        }
    }
}
