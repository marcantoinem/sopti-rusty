use super::{course_type::CourseType, group::Group, group_index::GroupIndex};
use compact_str::CompactString;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Course {
    pub sigle: CompactString,
    pub name: String,
    pub course_type: CourseType,
    pub nb_credit: usize,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CourseName {
    pub sigle: String,
    pub name: String,
    pub nb_credit: u8,
}

impl From<&Course> for CourseName {
    fn from(value: &Course) -> Self {
        Self {
            sigle: value.sigle.to_string(),
            name: value.name.to_string(),
            nb_credit: value.nb_credit as u8,
        }
    }
}

impl Course {
    pub fn new(sigle: &str, name: &str, nb_credit: usize, course_type: &str) -> Self {
        Course {
            sigle: sigle.into(),
            name: name.to_string(),
            course_type: course_type.into(),
            nb_credit,
        }
    }
    pub fn insert_or_push(&mut self, period_type: &str, new_group: Group) {
        match (period_type, &mut self.course_type) {
            ("C", CourseType::TheoOnly { theo_groups }) => theo_groups.insert_or_push(new_group),
            ("L", CourseType::LabOnly { lab_groups }) => lab_groups.insert_or_push(new_group),
            ("C", CourseType::Both { theo_groups, .. }) => theo_groups.insert_or_push(new_group),
            ("L", CourseType::Both { lab_groups, .. }) => lab_groups.insert_or_push(new_group),
            ("C", CourseType::Linked { theo_groups, .. }) => theo_groups.insert_or_push(new_group),
            ("L", CourseType::Linked { lab_groups, .. }) => lab_groups.insert_or_push(new_group),
            _ => (),
        }
    }
    pub fn get_mut(&mut self, period_type: &str, number: GroupIndex) -> Option<&mut Group> {
        match (period_type, &mut self.course_type) {
            ("C", CourseType::TheoOnly { theo_groups }) => theo_groups.get_mut(number),
            ("L", CourseType::LabOnly { lab_groups }) => lab_groups.get_mut(number),
            ("C", CourseType::Both { theo_groups, .. }) => theo_groups.get_mut(number),
            ("L", CourseType::Both { lab_groups, .. }) => lab_groups.get_mut(number),
            ("C", CourseType::Linked { theo_groups, .. }) => theo_groups.get_mut(number),
            ("L", CourseType::Linked { lab_groups, .. }) => lab_groups.get_mut(number),
            _ => None,
        }
    }
    pub fn for_each_groups(&self, function: impl FnMut(&Group)) {
        match &self.course_type {
            CourseType::LabOnly { lab_groups } => lab_groups.iter().for_each(function),
            CourseType::TheoOnly { theo_groups } => theo_groups.iter().for_each(function),
            CourseType::Linked {
                theo_groups,
                lab_groups,
            }
            | CourseType::Both {
                theo_groups,
                lab_groups,
            } => theo_groups
                .iter()
                .chain(lab_groups.iter())
                .for_each(function),
        }
    }

    pub fn for_each_groups_mut(&mut self, function: impl FnMut(&mut Group)) {
        match &mut self.course_type {
            CourseType::LabOnly { lab_groups } => lab_groups.iter_mut().for_each(function),
            CourseType::TheoOnly { theo_groups } => theo_groups.iter_mut().for_each(function),
            CourseType::Linked {
                theo_groups,
                lab_groups,
            }
            | CourseType::Both {
                theo_groups,
                lab_groups,
            } => theo_groups
                .iter_mut()
                .chain(lab_groups.iter_mut())
                .for_each(function),
        }
    }
}
