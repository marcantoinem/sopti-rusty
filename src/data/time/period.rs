use super::{
    day::Day,
    hours::{Hours, NO_HOUR},
};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Period {
    pub day: Day,
    pub room: Rc<str>,
    pub hours: Hours,
}

impl Period {
    pub fn new(day: &str, room: Rc<str>, hours: &str) -> Self {
        Self {
            day: day.into(),
            room,
            hours: hours.into(),
        }
    }
    fn overlap(&self, other: &Self) -> bool {
        self.hours & other.hours != NO_HOUR
    }
}
