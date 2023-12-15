use super::course::{Course, CourseName};
use super::group::Group;
use super::time::period::Period;
use compact_str::CompactString;
use std::{array, collections::HashMap, io::BufRead};

#[derive(Debug, Clone)]
pub struct Courses {
    courses: HashMap<CompactString, Course>,
}

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
            let [_, Some(sigle), Some(number), Some(nb_credit), _, _, Some(room), Some(course_type), _, Some(week_nb), _, Some(name), _, Some(week_day), Some(hour)] =
                array::from_fn(|_| columns.next())
            else {
                continue;
            };
            if let Some(course) = self.courses.get_mut(sigle) {
                let period = Period::new(week_day, room.into(), hour, week_nb);
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
                else {
                    continue;
                };
                let mut course = Course::new(sigle.into(), name.into(), nb_credit);
                let period = Period::new(week_day, room.into(), hour, week_nb);
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
                array::from_fn(|_| columns.next())
            else {
                continue;
            };
            let Some(course) = self.courses.get_mut(sigle) else {
                continue;
            };
            let Ok(number) = number.parse() else { continue };
            let groups = match course_type {
                "L" => &mut course.lab_groups,
                "C" => &mut course.theo_groups,
                _ => continue,
            };
            groups.get_mut(number).and_then(|g| Some(g.open = false));
        }
    }

    pub fn get_courses_name(&self) -> Vec<CourseName> {
        let mut courses_name: Vec<_> = self.courses.values().map(|c| c.into()).collect();
        courses_name.sort_unstable();
        courses_name
    }

    pub fn get_course(&self, sigle: &str) -> Option<Course> {
        self.courses.get(sigle).cloned()
    }
}
