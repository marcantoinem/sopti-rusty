use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[repr(u8)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum WeekNumber {
    B1 = 0,
    B2 = 1,
    Both = 2,
}

impl Display for WeekNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::B1 => f.write_str("B1"),
            Self::B2 => f.write_str("B2"),
            Self::Both => f.write_str(""),
        }
    }
}

impl From<&str> for WeekNumber {
    fn from(week_nb: &str) -> Self {
        match week_nb {
            "I" => WeekNumber::B1,
            "P" => WeekNumber::B2,
            _ => WeekNumber::Both,
        }
    }
}
