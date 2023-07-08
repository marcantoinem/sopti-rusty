use super::schedule::Schedule;
use super::{course::Course, schedule_course::TakenCourse};
use std::{collections::HashMap, io::BufRead};

#[derive(Debug, Clone)]
pub struct Courses {
    courses: HashMap<String, Course>,
}

impl Courses {
    pub fn from_csv(csv_courses: impl BufRead, csv_periods: impl BufRead) -> Courses {
        let mut lines = csv_courses.lines();
        lines.next();
        let mut courses = HashMap::new();
        // Create courses and the group with void Schedule
        for line in lines {
            let Ok(line) = line else {continue};
            let mut element = line.split(';');
            element.next();
            let Some(sigle) = element.next() else {continue};
            let sigle = sigle.to_string();
            let Some(group) = element.next() else {continue};
            let group = group.to_string();
            // let course_type = match element.next() {
            //     Some("C") => CourseType::Theo,
            //     Some("L") => CourseType::Lab,
            //     Some(_) | None => continue,
            // };
            // let Some(nb_students) = element.next() else {continue};
            // let Ok(nb_students) = nb_students.parse::<usize>() else {continue};
            // let Some(capacity) = element.next() else {continue};
            // let Ok(capacity) = capacity.parse::<usize>() else {continue};
            // let course = Course {
            //     sigle: sigle.clone(),
            //     group,
            //     course_type,
            //     nb_students,
            //     capacity,
            // };
            // courses.insert(sigle, course);
            todo!();
        }
        Courses { courses }
    }

    fn get_schedules_with_rules(
        &self,
        courses_to_take: &[String],
        rule: impl Fn(&Schedule) -> bool,
    ) -> Vec<Schedule> {
        let courses_to_take: Vec<&Course> = courses_to_take
            .iter()
            .filter_map(|name| self.courses.get(name))
            .collect();
        let mut schedules = vec![];
        self.get_schedules_rec(Schedule::default(), &courses_to_take, &mut schedules, rule);
        schedules
    }

    fn get_schedules_rec(
        &self,
        courses_taken: Schedule,
        courses: &[&Course],
        schedules: &mut Vec<Schedule>,
        rule: impl Fn(&Schedule) -> bool,
    ) {
        let Some(course) = courses.first() else {
            schedules.push(courses_taken);
            return;
        };
        if course.theo_groups.is_empty() {
            for lab_group in &course.lab_groups {
                let course = TakenCourse::from(course, None, Some(lab_group.clone()));
                let courses_taken = courses_taken.clone().add(course);
                if rule(&courses_taken) {
                    self.get_schedules_rec(courses_taken, &courses[1..], schedules, &rule);
                }
            }
        }
        for theo_group in &course.theo_groups {
            if course.lab_groups.is_empty() {
                let course = TakenCourse::from(course, Some(theo_group.clone()), None);
                let courses_taken = courses_taken.clone().add(course);
                if rule(&courses_taken) {
                    self.get_schedules_rec(courses_taken, &courses[1..], schedules, &rule);
                }
            }
            for lab_group in &course.lab_groups {
                let course =
                    TakenCourse::from(course, Some(theo_group.clone()), Some(lab_group.clone()));
                let courses_taken = courses_taken.clone().add(course);
                if rule(&courses_taken) {
                    self.get_schedules_rec(courses_taken, &courses[1..], schedules, &rule);
                }
            }
        }
    }
}
