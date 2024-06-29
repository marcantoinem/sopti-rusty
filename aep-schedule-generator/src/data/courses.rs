use super::course::{Course, CourseName};
use super::group::Group;
use super::group_index::GroupIndex;
use super::group_sigle::SigleGroup;
use super::time::period::{Period, PeriodCourse};
use compact_str::CompactString;
use std::collections::HashSet;
use std::{array, collections::HashMap, io::BufRead};

#[derive(Debug, Clone)]
pub struct Courses {
    courses: HashMap<CompactString, Course>,
    rooms: HashMap<CompactString, Vec<PeriodCourse>>,
}

impl Courses {
    pub fn from_csv(csv_horsages: impl BufRead, csv_fermes: impl BufRead) -> Courses {
        let courses = HashMap::new();
        let rooms = HashMap::new();
        let mut courses = Courses { courses, rooms };
        courses.update_all_courses(csv_horsages);
        courses.update_closed(csv_fermes);
        courses
    }

    fn update_all_courses(&mut self, csv_horsages: impl BufRead) {
        let mut lines = csv_horsages.lines();
        lines.next();
        for line in lines {
            let Ok(line) = line else { continue };
            let mut columns = line.split(';');
            let [_, Some(sigle), Some(number), Some(nb_credit), _, _, Some(room), Some(period_type), _, Some(week_nb), Some(course_type), Some(name), _, Some(week_day), Some(hour)] =
                array::from_fn(|_| columns.next())
            else {
                continue;
            };
            if let Some(course) = self.courses.get_mut(sigle) {
                let period = Period::new(week_day, room.into(), hour, week_nb);
                course.insert_or_push(period_type, Group::new(number, period));
            } else {
                let Ok(nb_credit) = nb_credit
                    .chars()
                    .take_while(|c| c.is_numeric())
                    .collect::<String>()
                    .parse::<usize>()
                else {
                    continue;
                };
                let mut course = Course::new(sigle.into(), name.into(), nb_credit, course_type);
                let period = Period::new(week_day, room.into(), hour, week_nb);
                course.insert_or_push(period_type, Group::new(number, period));
                self.courses.insert(sigle.into(), course);
            }
        }

        for course in self.courses.values() {
            course.for_each_groups(|group| {
                for period in group.periods.iter() {
                    self.rooms
                        .entry(period.room.clone())
                        .and_modify(|periods| {
                            periods.push(PeriodCourse::from(period, course.sigle.clone()))
                        })
                        .or_default();
                }
            });
        }
    }

    fn update_closed(&mut self, csv_fermes: impl BufRead) {
        let mut lines = csv_fermes.lines();
        lines.next();
        for line in lines {
            let Ok(line) = line else { continue };
            let mut columns = line.split(';');
            let [_, Some(sigle), Some(number), Some(period_type)] =
                array::from_fn(|_| columns.next())
            else {
                continue;
            };
            let Some(course) = self.courses.get_mut(sigle) else {
                continue;
            };
            let Ok(number) = number.parse::<u8>() else {
                continue;
            };
            let number = GroupIndex::from(number - 1);
            course
                .get_mut(period_type, number)
                .and_then(|g| Some(g.open = false));
        }
    }

    pub fn update(
        &mut self,
        csv_horsages: impl BufRead,
        csv_fermes: impl BufRead,
    ) -> HashSet<SigleGroup> {
        let closed: Vec<SigleGroup> = self
            .courses
            .iter()
            .map(|c| c.1.get_all_closed_groups())
            .flatten()
            .collect();
        self.update_all_courses(csv_horsages);
        self.update_closed(csv_fermes);
        closed
            .into_iter()
            .filter(|sigle_group| {
                self.courses
                    .get(&sigle_group.sigle)
                    .is_some_and(|course| course.is_open(sigle_group))
            })
            .collect()
    }

    pub fn get_courses_name(&self) -> Vec<CourseName> {
        let mut courses_name: Vec<_> = self.courses.values().map(|c| c.into()).collect();
        courses_name.sort_unstable();
        courses_name
    }

    pub fn get_course(&self, sigle: &str) -> Option<Course> {
        self.courses.get(sigle).cloned()
    }

    pub fn get_courses(&self, sigles: &[&str]) -> Vec<Course> {
        sigles
            .into_iter()
            .filter_map(|sigle| self.get_course(*sigle))
            .collect()
    }

    pub fn get_classroom(&self, local: &CompactString) -> Vec<PeriodCourse> {
        self.rooms.get(local).cloned().unwrap_or_default()
    }

    pub fn get_all_classroom(&self) -> Vec<CompactString> {
        let mut rooms: Vec<_> = self.rooms.keys().cloned().collect();
        rooms.sort_unstable();
        rooms
    }
}
