use super::course::Course;
use super::group::Group;
use super::time::period::Period;
use crate::algorithm::{schedule::Schedule, schedules::Schedules, taken_course::TakenCourse};
use compact_str::CompactString;
use std::{array, collections::HashMap, io::BufRead};

#[derive(Debug, Clone)]
pub struct Courses {
    courses: HashMap<CompactString, Course>,
}

pub static mut COUNTER: usize = 0;

impl Courses {
    pub fn from_csv(csv_horsages: impl BufRead, csv_fermes: impl BufRead) -> Courses {
        let courses = HashMap::new();
        let mut courses = Courses { courses };
        courses.update_all_courses(csv_horsages);
        courses.update_closed(csv_fermes);
        courses
    }

    pub fn update_all_courses(&mut self, csv_horsages: impl BufRead) {
        let mut lines = csv_horsages.lines();
        lines.next();
        for line in lines {
            let Ok(line) = line else { continue };
            let mut columns = line.split(';');
            let [_, Some(sigle), Some(number), Some(nb_credit), _, _, Some(room), Some(course_type), _, _, _, Some(name), _, Some(week_day), Some(hour)] =
                array::from_fn(|_| columns.next())
            else {
                continue;
            };
            if let Some(course) = self.courses.get_mut(sigle) {
                let period = Period::new(week_day, room.into(), hour);
                let groups = match course_type {
                    "L" => &mut course.lab_groups,
                    "C" => &mut course.theo_groups,
                    _ => continue,
                };
                groups.insert_or_push(Group::new(number, period));
            } else {
                let Ok(nb_credit) = nb_credit
                    .chars()
                    .take_while(|c| c.is_numeric())
                    .collect::<String>()
                    .parse::<usize>()
                    else {continue};
                let mut course = Course::new(sigle.into(), name.into(), nb_credit);
                let period = Period::new(week_day, room.into(), hour);
                let groups = match course_type {
                    "L" => &mut course.lab_groups,
                    "C" => &mut course.theo_groups,
                    _ => continue,
                };
                groups.insert_or_push(Group::new(number, period));
                self.courses.insert(sigle.into(), course);
            }
        }
    }
    pub fn update_closed(&mut self, csv_fermes: impl BufRead) {
        let mut lines = csv_fermes.lines();
        lines.next();
        for line in lines {
            let Ok(line) = line else { continue };
            let mut columns = line.split(';');
            let [_, Some(sigle), Some(number), Some(course_type)] =
                array::from_fn(|_| columns.next()) else { continue };
            let Some(course) = self.courses.get_mut(sigle) else { continue };
            let Ok(number) = number.parse() else { continue };
            let groups = match course_type {
                "L" => &mut course.lab_groups,
                "C" => &mut course.theo_groups,
                _ => continue,
            };
            groups.get_mut(number).and_then(|g| Some(g.closed = true));
        }
    }

    pub fn get_schedules(
        &self,
        courses_to_take: &[&str],
        rule: impl Fn(&Schedule, &TakenCourse) -> bool,
        evaluation: impl Fn(&Schedule) -> f64,
        nb_schedule: usize,
    ) -> Schedules {
        let courses_to_take: Vec<&Course> = courses_to_take
            .iter()
            .filter_map(|name| self.courses.get(*name))
            .collect();
        let mut schedules = Schedules::new(nb_schedule);
        self.get_schedules_rec(
            Schedule::default(),
            &courses_to_take,
            &mut schedules,
            &rule,
            &evaluation,
        );
        schedules
    }
    fn get_schedules_rec(
        &self,
        courses_taken: Schedule,
        courses: &[&Course],
        schedules: &mut Schedules,
        rule: &impl Fn(&Schedule, &TakenCourse) -> bool,
        evaluation: &impl Fn(&Schedule) -> f64,
    ) {
        unsafe {
            COUNTER += 1;
        }
        let Some(course) = courses.first() else {
            schedules.push(courses_taken, evaluation);
            return;
        };

        if course.theo_groups.is_empty() {
            for lab_group in course.lab_groups.iter() {
                let course = TakenCourse::new(course, None, Some(lab_group.clone()));
                if rule(&courses_taken, &course) {
                    let courses_taken = courses_taken.clone().add(course);
                    self.get_schedules_rec(
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
                    self.get_schedules_rec(
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
                    self.get_schedules_rec(
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
