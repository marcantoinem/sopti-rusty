use super::groups::Groups;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Course {
    pub sigle: Rc<str>,
    pub name: Rc<str>,
    pub theo_groups: Groups,
    pub lab_groups: Groups,
    pub nb_credit: usize,
}

impl Course {
    pub fn new(sigle: impl AsRef<str>, name: impl AsRef<str>, nb_credit: usize) -> Self {
        Course {
            sigle: Rc::from(sigle.as_ref()),
            name: Rc::from(name.as_ref()),
            theo_groups: Groups::default(),
            lab_groups: Groups::default(),
            nb_credit,
        }
    }
}
