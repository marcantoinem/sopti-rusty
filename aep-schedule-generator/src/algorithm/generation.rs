use compact_str::CompactString;

use super::{
    //conflicts::Conflicts,
    schedule::ScheduleBuilder,
    schedules::Schedules,
    scores::EvaluationOption,
};
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
    // pub fn get_simple_conflict<'a>(&'a self) -> Option<Conflicts> {

    // }
    pub fn apply_personal_schedule<'a>(&'a mut self) {
        self.courses_to_take
            .iter_mut()
            .for_each(|c| c.apply_week_mask(&self.user_conflicts));
    }
    pub fn get_impossible_course<'a>(&'a self) -> Vec<CompactString> {
        self.courses_to_take
            .iter()
            .filter(|c| c.is_impossible())
            .map(|c| c.sigle.clone())
            .collect()
    }
    pub fn get_nb_combinations<'a>(&'a self) -> usize {
        self.courses_to_take
            .iter()
            .map(Course::nb_combinations)
            .product()
    }
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
