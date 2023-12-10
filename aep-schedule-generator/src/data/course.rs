use super::groups::Groups;
use compact_str::CompactString;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Course {
    pub sigle: CompactString,
    pub name: Rc<str>,
    pub theo_groups: Groups,
    pub lab_groups: Groups,
    pub nb_credit: usize,
}

impl Course {
    pub fn new(sigle: &str, name: Rc<str>, nb_credit: usize) -> Self {
        Course {
            sigle: sigle.into(),
            name,
            theo_groups: Groups::default(),
            lab_groups: Groups::default(),
            nb_credit,
        }
    }
}
