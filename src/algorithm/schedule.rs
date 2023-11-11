use super::taken_course::TakenCourse;
use crate::data::time::week::Week;
use std::cmp::Ordering;

/// It reverse the order of the value to make the heap a min heap
#[derive(PartialEq, Debug, Clone, Default)]
pub struct Schedule {
    pub value: f64,
    week: Week,
    courses: Vec<TakenCourse>,
}

impl PartialOrd for Schedule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.value.partial_cmp(&self.value)
    }
}

/// Trust me bro (I expect the evaluation function to not have stupid NaN value)
impl Eq for Schedule {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Ord for Schedule {
    fn cmp(&self, other: &Self) -> Ordering {
        other.value.partial_cmp(&self.value).unwrap()
    }
}

impl Schedule {
    pub fn add(mut self, course: TakenCourse) -> Schedule {
        if let Some(theo_group) = &course.theo_group {
            for period in &theo_group.periods {
                self.week.add_period(period);
            }
        }
        if let Some(lab_group) = &course.lab_group {
            for period in &lab_group.periods {
                self.week.add_period(period);
            }
        }
        self.courses.push(course);
        self
    }
    pub fn allow_conflicts(&self, _new_course: &TakenCourse) -> bool {
        true
    }
    pub fn forbid_conflicts(&self, new_course: &TakenCourse) -> bool {
        let mut new_week = self.week.clone();
        if let Some(theo_group) = &new_course.theo_group {
            for period in &theo_group.periods {
                if self.week.conflict_in_day(period) {
                    return false;
                }
                new_week.add_period(period);
            }
        }
        if let Some(lab_group) = &new_course.lab_group {
            for period in &lab_group.periods {
                if self.week.conflict_in_day(period) {
                    return false;
                }
                new_week.add_period(period);
            }
        }
        true
    }
    pub fn more_morning(&self) -> f64 {
        0.0
    }
}
