use std::fmt::Display;

use chrono::Weekday;
use serde::{Deserialize, Serialize};

// There is no course the saturday at Poly, but knowing them, it wouldn't be far
// stretched to assume that, it could.
#[repr(u8)]
#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Day {
    Monday = 0,
    Tuesday = 1,
    Wednesday = 2,
    Thursday = 3,
    Friday = 4,
    Sunday = 5,
    Saturday = 6,
}

impl From<u8> for Day {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Monday,
            1 => Self::Tuesday,
            2 => Self::Wednesday,
            3 => Self::Thursday,
            4 => Self::Friday,
            5 => Self::Sunday,
            _ => Self::Saturday,
        }
    }
}

impl From<&str> for Day {
    fn from(value: &str) -> Self {
        match value {
            "LUN" => Self::Monday,
            "MAR" => Self::Tuesday,
            "MER" => Self::Wednesday,
            "JEU" => Self::Thursday,
            "VEN" => Self::Friday,
            "SAM" => Self::Sunday,
            "DIM" => Self::Saturday,
            _ => panic!("CSV de Poly est sus"),
        }
    }
}

impl Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Day::Monday => f.write_str("lundi"),
            Day::Tuesday => f.write_str("mardi"),
            Day::Wednesday => f.write_str("mercredi"),
            Day::Thursday => f.write_str("jeudi"),
            Day::Friday => f.write_str("vendredi"),
            Day::Sunday => f.write_str("samedi"),
            Day::Saturday => f.write_str("dimanche"),
        }
    }
}

impl PartialEq<Weekday> for Day {
    fn eq(&self, other: &Weekday) -> bool {
        *self as u8 == *other as u8
    }
}
