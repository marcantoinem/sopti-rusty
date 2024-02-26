use crate::data::course::Course;
use crate::data::group::Group;
use crate::data::group_index::GroupIndex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TakenCourse {
    index: u8,
    theo_group: GroupIndex,
    lab_group: GroupIndex,
}

impl TakenCourse {
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
}
