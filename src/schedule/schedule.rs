use super::schedule_course::TakenCourse;
use crate::period::Period;

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Schedule {
    courses: Vec<TakenCourse>,
}

impl Schedule {
    pub fn new(courses: Vec<TakenCourse>) -> Schedule {
        Self { courses }
    }
    pub fn add(mut self, course: TakenCourse) -> Schedule {
        self.courses.push(course);
        self
    }
    fn periods(&self) -> Vec<Period> {
        let lab_groups = self
            .courses
            .iter()
            .filter_map(|c| c.lab_group.clone())
            .flat_map(|c| c.periods);
        let theo_groups = self
            .courses
            .iter()
            .filter_map(|c| c.theo_group.clone())
            .flat_map(|c| c.periods);
        let mut periods: Vec<Period> = lab_groups.chain(theo_groups).collect();
        periods.sort_unstable();
        periods
    }
    pub fn allow_conflicts(&self) -> bool {
        true
    }
    pub fn forbid_conflicts(&self) -> bool {
        let periods = self.periods();
        (0..periods.len() - 1)
            .into_iter()
            .all(|i| !periods[i].is_overlapping(&periods[i + 1]))
    }
    pub fn more_morning(&self) -> f64 {
        let periods = self.periods();
        let mut first_hours = [0; 7];
        for p in periods {
            if first_hours[p.day as usize] < p.hour {
                first_hours[p.day as usize] = p.hour;
            }
        }
        first_hours.iter().map(|h| *h as f64).sum()
    }
}
