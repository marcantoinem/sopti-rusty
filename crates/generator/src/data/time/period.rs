use super::{day::Day, hours::Hours, week_number::WeekNumber};
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Period {
    pub day: Day,
    pub room: CompactString,
    pub hours: Hours,
    pub week_nb: WeekNumber,
}

impl Period {
    pub fn new(day: &str, room: &str, hours: &str, week_nb: &str) -> Self {
        Self {
            day: day.into(),
            room: room.into(),
            hours: hours.into(),
            week_nb: week_nb.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeriodCourse {
    pub period: Period,
    pub sigle: CompactString,
}

impl PeriodCourse {
    pub fn from(period: &Period, sigle: CompactString) -> Self {
        Self {
            period: period.clone(),
            sigle: sigle.clone(),
        }
    }
}
