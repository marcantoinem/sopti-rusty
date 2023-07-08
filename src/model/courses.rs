use super::course::Course;
use super::schedule::Schedule;
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

    pub fn get_schedules(&self, course_to_take: &[String]) -> Vec<Schedule> {
        vec![]
    }

    fn get_schedules_rec(&self, course_to_take: &[String], schedules: &mut Vec<Schedule>) {
        // self.get_schedules_rec(, schedules);
    }
}
