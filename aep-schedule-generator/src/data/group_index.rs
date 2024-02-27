use crate::data::group::Group;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct GroupIndex(u8);

impl Default for GroupIndex {
    fn default() -> Self {
        Self(u8::MAX)
    }
}

impl From<u8> for GroupIndex {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<usize> for GroupIndex {
    fn from(value: usize) -> Self {
        Self(value as u8)
    }
}

impl From<&Group> for GroupIndex {
    fn from(value: &Group) -> Self {
        value.number
    }
}

impl GroupIndex {
    pub fn value(self) -> Option<u8> {
        match self {
            Self(u8::MAX) => None,
            value => Some(value.0),
        }
    }
    pub fn to_usize(self) -> usize {
        self.0 as usize
    }
    pub fn none() -> Self {
        Self(u8::MAX)
    }
}
