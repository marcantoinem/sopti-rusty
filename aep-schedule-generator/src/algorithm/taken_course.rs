use crate::data::course::Course;
use crate::data::group::Group;
use compact_str::CompactString;
use std::fmt::Display;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TakenCourse {
    pub sigle: CompactString,
    pub name: Rc<str>,
    pub theo_group: Option<Group>,
    pub lab_group: Option<Group>,
    pub nb_credit: usize,
}

impl Display for TakenCourse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut to_print = String::new();
        to_print.push_str(&self.sigle);
        to_print.push_str(": ");
        to_print.push_str(&format!("{: <30}", self.name));
        if let Some(theo_group) = &self.theo_group {
            to_print.push_str(" theo: ");
            to_print.push_str(&theo_group.number.to_string());
        }
        if let Some(lab_group) = &self.lab_group {
            to_print.push_str(" lab: ");
            to_print.push_str(&lab_group.number.to_string());
        }
        writeln!(f, "{}", to_print)
    }
}

impl TakenCourse {
    pub fn new(course: &Course, theo_group: Option<Group>, lab_group: Option<Group>) -> Self {
        Self {
            sigle: course.sigle.clone(),
            name: course.name.clone(),
            theo_group,
            lab_group,
            nb_credit: course.nb_credit,
        }
    }
}
