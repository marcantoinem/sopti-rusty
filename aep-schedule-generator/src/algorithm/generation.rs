use super::{schedule::Schedule, schedules::Schedules, scores::EvaluationOption};
use crate::data::course::Course;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct SchedulesOptions {
    pub courses_to_take: Vec<Course>,
    pub max_nb_conflicts: u8,
    pub evaluation: EvaluationOption,
    pub max_size: usize,
}

impl SchedulesOptions {
    pub fn get_schedules<'a>(&'a self) -> Schedules<'a> {
        let mut schedules = Schedules::new(&self);
        if self.courses_to_take.len() == 0 {
            return schedules;
        }
        schedules.get_schedules_rec(
            Schedule::new(&self.courses_to_take),
            self.max_nb_conflicts,
            0,
        );
        schedules
    }
}
