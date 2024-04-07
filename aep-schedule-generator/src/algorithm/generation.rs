use super::{schedule::ScheduleBuilder, schedules::Schedules, scores::EvaluationOption};
use crate::data::{course::Course, time::week::Week};

#[derive(Default, Debug, Clone, PartialEq)]
pub struct SchedulesOptions {
    pub courses_to_take: Vec<Course>,
    pub max_nb_conflicts: u8,
    pub evaluation: EvaluationOption,
    pub max_size: usize,
    pub user_conflicts: Week<5>,
}

impl SchedulesOptions {
    pub fn get_schedules<'a>(&'a self) -> Schedules<'a> {
        let mut schedules = Schedules::new(&self);
        if self.courses_to_take.len() == 0 {
            return schedules;
        }
        schedules.get_schedules_rec(
            ScheduleBuilder::new(&self.courses_to_take),
            self.max_nb_conflicts,
            0,
        );
        schedules
    }
}
