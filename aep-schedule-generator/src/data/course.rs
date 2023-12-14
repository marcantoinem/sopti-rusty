use super::groups::Groups;
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Course {
    pub sigle: CompactString,
    pub name: String,
    pub theo_groups: Groups,
    pub lab_groups: Groups,
    pub nb_credit: usize,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CourseName {
    pub sigle: String,
    pub name: String,
    pub nb_credit: u8,
}

impl From<&Course> for CourseName {
    fn from(value: &Course) -> Self {
        Self {
            sigle: value.sigle.to_string(),
            name: value.name.to_string(),
            nb_credit: value.nb_credit as u8,
        }
    }
}

impl Course {
    pub fn new(sigle: &str, name: &str, nb_credit: usize) -> Self {
        Course {
            sigle: sigle.into(),
            name: name.to_string(),
            theo_groups: Groups::default(),
            lab_groups: Groups::default(),
            nb_credit,
        }
    }
}
