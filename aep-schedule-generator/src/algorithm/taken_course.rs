use crate::data::course::Course;
use crate::data::course_type::CourseType;
use crate::data::group::Group;
use crate::data::group_index::GroupIndex;
use serde::{Deserialize, Serialize};

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
    pub fn for_each_group<'a>(&self, courses: &'a [Course], function: impl FnMut(&Group)) {
        let course = self.get_course(courses);
        match &course.course_type {
            CourseType::LabOnly { lab_groups } => {
                lab_groups[self.lab_group].iter().for_each(function)
            }
            CourseType::TheoOnly { theo_groups } => {
                theo_groups[self.theo_group].iter().for_each(function)
            }
            CourseType::Linked {
                theo_groups,
                lab_groups,
            }
            | CourseType::Both {
                theo_groups,
                lab_groups,
            } => theo_groups[self.theo_group]
                .iter()
                .chain(lab_groups[self.lab_group].iter())
                .for_each(function),
        }
    }
    pub fn build<'a>(self, courses: &'a [Course]) -> TakenCourse {
        let course = self.get_course(courses).clone();
        let taken_course_type = match &course.course_type {
            CourseType::LabOnly { lab_groups } => TakenCourseType::LabOnly {
                lab_group: lab_groups[self.lab_group].clone().unwrap(),
            },
            CourseType::TheoOnly { theo_groups } => TakenCourseType::TheoOnly {
                theo_group: theo_groups[self.theo_group].clone().unwrap(),
            },
            CourseType::Linked {
                theo_groups,
                lab_groups,
            } => TakenCourseType::Linked {
                theo_group: theo_groups[self.theo_group].clone().unwrap(),
                lab_group: lab_groups[self.lab_group].clone().unwrap(),
            },
            CourseType::Both {
                theo_groups,
                lab_groups,
            } => TakenCourseType::Both {
                theo_group: theo_groups[self.theo_group].clone().unwrap(),
                lab_group: lab_groups[self.lab_group].clone().unwrap(),
            },
        };

        TakenCourse {
            sigle: course.sigle.to_string(),
            name: course.name,
            taken_course_type,
            nb_credit: course.nb_credit,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum TakenCourseType {
    TheoOnly { theo_group: Group },
    LabOnly { lab_group: Group },
    Both { theo_group: Group, lab_group: Group },
    Linked { theo_group: Group, lab_group: Group },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct TakenCourse {
    pub sigle: String,
    pub name: String,
    pub taken_course_type: TakenCourseType,
    pub nb_credit: usize,
}

impl TakenCourse {
    pub fn theo_group(&self) -> Option<&Group> {
        match &self.taken_course_type {
            TakenCourseType::TheoOnly { theo_group }
            | TakenCourseType::Linked { theo_group, .. }
            | TakenCourseType::Both { theo_group, .. } => Some(theo_group),
            _ => None,
        }
    }
    pub fn lab_group(&self) -> Option<&Group> {
        match &self.taken_course_type {
            TakenCourseType::LabOnly { lab_group }
            | TakenCourseType::Linked { lab_group, .. }
            | TakenCourseType::Both { lab_group, .. } => Some(lab_group),
            _ => None,
        }
    }
    pub fn for_each_group(&self, mut function: impl FnMut(&Group)) {
        match &self.taken_course_type {
            TakenCourseType::LabOnly { lab_group } => function(lab_group),
            TakenCourseType::TheoOnly { theo_group } => function(theo_group),
            TakenCourseType::Linked {
                theo_group,
                lab_group,
            }
            | TakenCourseType::Both {
                theo_group,
                lab_group,
            } => {
                function(theo_group);
                function(lab_group)
            }
        }
    }
}
