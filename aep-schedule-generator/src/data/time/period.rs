use compact_str::CompactString;

use super::{day::Day, hours::Hours};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Period {
    pub day: Day,
    pub room: CompactString,
    pub hours: Hours,
}

impl Period {
    pub fn new(day: &str, room: &str, hours: &str) -> Self {
        Self {
            day: day.into(),
            room: room.into(),
            hours: hours.into(),
        }
    }
}
