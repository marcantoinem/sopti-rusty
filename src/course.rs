use super::group::Group;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Course {
    pub sigle: Rc<str>,
    pub name: Rc<str>,
    pub theo_groups: Vec<Group>,
    pub lab_groups: Vec<Group>,
    pub nb_credit: usize,
}

impl Course {
    pub fn new(sigle: impl AsRef<str>, name: impl AsRef<str>, nb_credit: usize) -> Self {
        Course {
            sigle: Rc::from(sigle.as_ref()),
            name: Rc::from(name.as_ref()),
            theo_groups: vec![],
            lab_groups: vec![],
            nb_credit,
        }
    }
}
