use super::{course::Course, group::Group};

#[derive(Debug, Clone)]
pub struct TakenCourse {
    pub sigle: String,
    pub theo_group: Option<Group>,
    pub lab_group: Option<Group>,
    pub nb_credit: usize,
}

impl TakenCourse {
    pub fn new(
        sigle: String,
        theo_group: Option<Group>,
        lab_group: Option<Group>,
        nb_credit: usize,
    ) -> TakenCourse {
        Self {
            sigle,
            theo_group,
            lab_group,
            nb_credit,
        }
    }
    pub fn from(
        course: &Course,
        theo_group: Option<Group>,
        lab_group: Option<Group>,
    ) -> TakenCourse {
        Self {
            sigle: course.sigle.clone(),
            theo_group,
            lab_group,
            nb_credit: course.nb_credit,
        }
    }
}
