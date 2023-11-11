use crate::data::course::Course;
use crate::data::group::Group;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TakenCourse {
    pub sigle: Rc<str>,
    pub name: Rc<str>,
    pub theo_group: Option<Group>,
    pub lab_group: Option<Group>,
    pub nb_credit: usize,
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
