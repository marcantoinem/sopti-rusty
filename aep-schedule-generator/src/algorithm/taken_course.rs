use serde::{Deserialize, Serialize};

use crate::data::course::Course;
use crate::data::group::Group;
use crate::data::group_index::GroupIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TakenCourseBuilder {
    index: u8,
    theo_group: GroupIndex,
    lab_group: GroupIndex,
    pub theo_group_conflict: bool,
    pub lab_group_conflict: bool,
}

impl TakenCourseBuilder {
    #[inline(always)]
    pub fn new(index: u8, theo_group: GroupIndex, lab_group: GroupIndex) -> Self {
        Self {
            index,
            theo_group,
            lab_group,
            theo_group_conflict: false,
            lab_group_conflict: false,
        }
    }
    #[inline(always)]
    pub fn get_course<'a>(&self, courses: &'a [Course]) -> &'a Course {
        &courses[self.index as usize]
    }
    #[inline(always)]
    pub fn get_lab_group<'a>(&self, courses: &'a [Course]) -> Option<&'a Group> {
        let course = self.get_course(courses);
        match self.lab_group.value() {
            Some(_) => course.lab_groups[self.lab_group].as_ref(),
            None => None,
        }
    }
    #[inline(always)]
    pub fn get_theo_group<'a>(&self, courses: &'a [Course]) -> Option<&'a Group> {
        let course = self.get_course(courses);
        match self.lab_group.value() {
            Some(_) => course.theo_groups[self.theo_group].as_ref(),
            None => None,
        }
    }
    pub fn build<'a>(self, courses: &'a [Course]) -> TakenCourse {
        let course = self.get_course(courses).clone();
        let mut lab_group = self.get_lab_group(courses).cloned();
        if let Some(lab_group) = &mut lab_group {
            lab_group.conflict = self.lab_group_conflict;
        }
        let mut theo_group = self.get_theo_group(courses).cloned();
        if let Some(theo_group) = &mut theo_group {
            theo_group.conflict = self.theo_group_conflict;
        }

        TakenCourse {
            sigle: course.sigle.to_string(),
            name: course.name,
            theo_group,
            lab_group,
            nb_credit: course.nb_credit,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TakenCourse {
    pub sigle: String,
    pub name: String,
    pub theo_group: Option<Group>,
    pub lab_group: Option<Group>,
    pub nb_credit: usize,
}
