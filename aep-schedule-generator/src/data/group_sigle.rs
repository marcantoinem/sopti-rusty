use std::fmt::Display;

use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use super::group_index::GroupIndex;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[repr(u8)]
pub enum GroupType {
    LabGroup,
    TheoGroup,
}

impl Display for GroupType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LabGroup => f.write_str("laboratoire"),
            Self::TheoGroup => f.write_str("théorie"),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SigleGroup {
    pub sigle: CompactString,
    pub group_type: GroupType,
    pub group_index: GroupIndex,
}

impl SigleGroup {
    pub fn new(sigle: CompactString, group_type: GroupType, group_index: GroupIndex) -> Self {
        Self {
            sigle,
            group_type,
            group_index,
        }
    }
}
