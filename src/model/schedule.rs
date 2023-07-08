use super::{period::Period, schedule_course::TakenCourse};

#[derive(Debug, Clone, Default)]
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
    pub fn allow_conflicts(&self) -> bool {
        true
    }
    pub fn forbid_conflicts(&self) -> bool {
        let mut periods: Vec<Period> = self
            .courses
            .iter()
            .filter_map(|c| c.lab_group.clone())
            .flat_map(|c| c.periods)
            .chain(
                self.courses
                    .iter()
                    .filter_map(|c| c.theo_group.clone())
                    .flat_map(|c| c.periods),
            )
            .collect();
        periods.sort_unstable();
        (0..periods.len() - 1)
            .into_iter()
            .any(|i| periods[i].is_overlapping(&periods[i + 1]))
    }
}
