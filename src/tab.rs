use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Sequence)]
pub enum Tab {
    #[default]
    Home,
    Projects,
    Deployments,
}

impl fmt::Display for Tab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self))
    }
}
