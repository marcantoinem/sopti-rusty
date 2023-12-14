use super::{schedule::Schedule, schedules::Schedules, taken_course::TakenCourse};
use crate::data::course::Course;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct SchedulesOptions {
    pub courses_to_take: Vec<Course>,
    pub max_nb_conflicts: u8,
}

impl SchedulesOptions {
    pub fn get_schedules(&self) -> Schedules {
        let mut schedules = Schedules::new(50);
        if self.courses_to_take.len() == 0 {
            return schedules;
        }
        Self::get_schedules_rec(
            Schedule::default(),
            &self.courses_to_take,
            &mut schedules,
            self.max_nb_conflicts,
        );
        schedules
    }
    fn get_schedules_rec(
        courses_taken: Schedule,
        courses: &[Course],
        schedules: &mut Schedules,
        n: u8,
    ) {
        let Some(course) = courses.first() else {
            schedules.push(courses_taken);
            return;
        };

        match (course.theo_groups.is_empty(), course.lab_groups.is_empty()) {
            (false, false) => {
                for theo_group in course.theo_groups.iter().filter(|g| g.open) {
                    for lab_group in course.lab_groups.iter().filter(|g| g.open) {
                        let course = TakenCourse::new(
                            course,
                            Some(theo_group.clone()),
                            Some(lab_group.clone()),
                        );
                        if let Some(schedule) = courses_taken.add_check_conflicts(n, &course) {
                            Self::get_schedules_rec(schedule, &courses[1..], schedules, n);
                        }
                    }
                }
            }
            (false, true) => {
                for theo_group in course.theo_groups.iter().filter(|g| g.open) {
                    let course = TakenCourse::new(course, Some(theo_group.clone()), None);
                    if let Some(schedule) = courses_taken.add_check_conflicts(n, &course) {
                        Self::get_schedules_rec(schedule, &courses[1..], schedules, n);
                    }
                }
            }
            (true, false) => {
                for lab_group in course.lab_groups.iter().filter(|g| g.open) {
                    let course = TakenCourse::new(course, None, Some(lab_group.clone()));
                    if let Some(schedule) = courses_taken.add_check_conflicts(n, &course) {
                        Self::get_schedules_rec(schedule, &courses[1..], schedules, n);
                    }
                }
            }
            _ => (),
        }
    }
}
