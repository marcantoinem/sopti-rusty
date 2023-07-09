use crate::model::group::Group;

use super::period::Period;
use super::schedule::Schedule;
use super::{course::Course, schedule_course::TakenCourse};
use std::{array, collections::HashMap, io::BufRead};

#[derive(Debug, Clone)]
pub struct Courses {
    courses: HashMap<String, Course>,
}

impl Courses {
    pub fn from_csv(csv_horsages: impl BufRead, csv_fermes: impl BufRead) -> Courses {
        let mut courses = HashMap::<String, Course>::new();
        let mut lines = csv_horsages.lines();
        lines.next();
        for line in lines {
            let Ok(line) = line else {continue};
            let mut columns = line.split(';');
            let [_, Some(sigle), Some(group), Some(nb_credit), _, _, Some(room), Some(course_type), _, _, _, Some(name), _, Some(week_day), Some(hour)] = array::from_fn(|_| columns.next()) else {continue};
            if let Some(course) = courses.get_mut(sigle) {
                let Ok(hour) = hour.parse::<usize>() else {continue};
                let period = Period::new(week_day, hour, room);
                match course_type {
                    "L" => {
                        if let Some(g) = course.lab_groups.iter_mut().find(|g| g.name == group) {
                            g.periods.push(period);
                        } else {
                            course.lab_groups.push(Group::new(group, period));
                        }
                    }
                    "C" => {
                        if let Some(g) = course.theo_groups.iter_mut().find(|g| g.name == group) {
                            g.periods.push(period);
                        } else {
                            course.theo_groups.push(Group::new(group, period));
                        }
                    }
                    _ => continue,
                }
            } else {
                let Ok(nb_credit) = nb_credit.chars().take_while(|c| c.is_numeric()).collect::<String>().parse::<usize>() else {continue};
                let Ok(hour) = hour.parse::<usize>() else {continue};
                let mut course = Course::new(sigle, name, nb_credit);
                let period = Period::new(week_day, hour, room);
                let group = Group::new(group, period);
                match course_type {
                    "L" => course.lab_groups.push(group),
                    "C" => course.theo_groups.push(group),
                    _ => continue,
                }
                courses.insert(sigle.to_string(), course);
            }
        }
        let mut lines = csv_fermes.lines();
        lines.next();
        for line in lines {
            let Ok(line) = line else {continue};
            let mut columns = line.split(';');
            let [_, Some(sigle), Some(group), Some(course_type)] = array::from_fn(|_| columns.next()) else {continue};
            let Some(course) = courses.get_mut(sigle) else {continue};
            match course_type {
                "L" => {
                    let Some(group) = course.lab_groups.iter_mut().find(|g| g.name == group) else {continue};
                    group.closed = true;
                }
                "C" => {
                    let Some(group) = course.theo_groups.iter_mut().find(|g| g.name == group) else {continue};
                    group.closed = true;
                }
                _ => continue,
            }
        }
        Courses { courses }
    }

    pub fn get_schedules(
        &self,
        courses_to_take: &[&str],
        rule: impl Fn(&Schedule) -> bool,
    ) -> Vec<Schedule> {
        let courses_to_take: Vec<&Course> = courses_to_take
            .iter()
            .filter_map(|name| self.courses.get(*name))
            .collect();
        let mut schedules = vec![];
        self.get_schedules_rec(Schedule::default(), &courses_to_take, &mut schedules, &rule);
        schedules
    }

    fn get_schedules_rec(
        &self,
        courses_taken: Schedule,
        courses: &[&Course],
        schedules: &mut Vec<Schedule>,
        rule: &impl Fn(&Schedule) -> bool,
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
                    self.get_schedules_rec(courses_taken, &courses[1..], schedules, rule);
                }
            }
        }
        for theo_group in &course.theo_groups {
            if course.lab_groups.is_empty() {
                let course = TakenCourse::from(course, Some(theo_group.clone()), None);
                let courses_taken = courses_taken.clone().add(course);
                if rule(&courses_taken) {
                    self.get_schedules_rec(courses_taken, &courses[1..], schedules, rule);
                }
            }
            for lab_group in &course.lab_groups {
                let course =
                    TakenCourse::from(course, Some(theo_group.clone()), Some(lab_group.clone()));
                let courses_taken = courses_taken.clone().add(course);
                if rule(&courses_taken) {
                    self.get_schedules_rec(courses_taken, &courses[1..], schedules, rule);
                }
            }
        }
    }
}
