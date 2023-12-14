use super::{schedule::Schedule, schedules::Schedules, taken_course::TakenCourse};
use crate::data::course::Course;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct SchedulesOptions {
    pub courses_to_take: Vec<Course>,
}

impl SchedulesOptions {
    pub fn get_schedules(&self) -> Schedules {
        let mut schedules = Schedules::new(50);
        Self::get_schedules_rec(
            Schedule::default(),
            &self.courses_to_take,
            &mut schedules,
            &Schedule::allow_n_conflicts(1),
            &Schedule::more_day_off,
        );
        schedules
    }
    fn get_schedules_rec(
        courses_taken: Schedule,
        courses: &[Course],
        schedules: &mut Schedules,
        rule: &impl Fn(&Schedule, &TakenCourse) -> bool,
        evaluation: &impl Fn(&Schedule) -> f64,
    ) {
        let Some(course) = courses.first() else {
            schedules.push(courses_taken, evaluation);
            return;
        };

        if course.theo_groups.is_empty() {
            for lab_group in course.lab_groups.iter() {
                let course = TakenCourse::new(course, None, Some(lab_group.clone()));
                if rule(&courses_taken, &course) {
                    let courses_taken = courses_taken.clone().add(course);
                    Self::get_schedules_rec(
                        courses_taken,
                        &courses[1..],
                        schedules,
                        rule,
                        evaluation,
                    );
                }
            }
        }
        for theo_group in course.theo_groups.iter() {
            if course.lab_groups.is_empty() {
                let course = TakenCourse::new(course, Some(theo_group.clone()), None);
                if rule(&courses_taken, &course) {
                    let courses_taken = courses_taken.clone().add(course);
                    Self::get_schedules_rec(
                        courses_taken,
                        &courses[1..],
                        schedules,
                        rule,
                        evaluation,
                    );
                }
            }
            for lab_group in course.lab_groups.iter() {
                let course =
                    TakenCourse::new(course, Some(theo_group.clone()), Some(lab_group.clone()));
                if rule(&courses_taken, &course) {
                    let courses_taken = courses_taken.clone().add(course);
                    Self::get_schedules_rec(
                        courses_taken,
                        &courses[1..],
                        schedules,
                        rule,
                        evaluation,
                    );
                }
            }
        }
    }
}
