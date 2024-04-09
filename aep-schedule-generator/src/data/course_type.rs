use super::groups::Groups;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum CourseType {
    TheoOnly {
        theo_groups: Groups,
    },
    LabOnly {
        lab_groups: Groups,
    },
    Both {
        theo_groups: Groups,
        lab_groups: Groups,
    },
    Linked {
        theo_groups: Groups,
        lab_groups: Groups,
    },
}

impl From<&str> for CourseType {
    fn from(value: &str) -> Self {
        match value {
            "T" => Self::TheoOnly {
                theo_groups: Groups::default(),
            },
            "L" => Self::LabOnly {
                lab_groups: Groups::default(),
            },
            "TLS" => Self::Both {
                theo_groups: Groups::default(),
                lab_groups: Groups::default(),
            },
            "TL" => Self::Linked {
                theo_groups: Groups::default(),
                lab_groups: Groups::default(),
            },
            _ => unreachable!("Poly is doing sus things in the CSV"),
        }
    }
}
