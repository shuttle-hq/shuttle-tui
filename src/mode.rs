use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Sequence)]
pub enum Mode {
    #[default]
    Home,
    Projects,
    Deployments,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
