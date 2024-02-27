use serde::{Deserialize, Serialize};

use crate::data::course::Course;
use crate::data::group::Group;
use crate::data::group_index::GroupIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TakenCourseBuilder {
    index: u8,
    theo_group: GroupIndex,
    lab_group: GroupIndex,
}

impl TakenCourseBuilder {
    #[inline(always)]
    pub fn new(index: u8, theo_group: GroupIndex, lab_group: GroupIndex) -> Self {
        Self {
            index,
            theo_group,
            lab_group,
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
            Some(_) => course.lab_groups[self.theo_group].as_ref(),
            None => None,
        }
    }
    pub fn build<'a>(self, courses: &'a [Course]) -> TakenCourse {
        let course = self.get_course(courses).clone();
        let lab_group = self.get_lab_group(courses).cloned();
        let theo_group = self.get_theo_group(courses).cloned();

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
