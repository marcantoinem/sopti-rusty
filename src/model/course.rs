use super::group::Group;

#[derive(Debug, Clone)]
pub struct Course {
    pub sigle: String,
    name: String,
    pub theo_groups: Vec<Group>,
    pub lab_groups: Vec<Group>,
    pub nb_credit: usize,
}

impl Course {
    pub fn new(sigle: impl Into<String>, name: impl Into<String>, nb_credit: usize) -> Self {
        Course {
            sigle: sigle.into(),
            name: name.into(),
            theo_groups: vec![],
            lab_groups: vec![],
            nb_credit,
        }
    }
}
