use super::group::Group;

#[derive(Debug, Clone)]
pub struct Course {
    pub sigle: String,
    pub theo_groups: Vec<Group>,
    pub lab_groups: Vec<Group>,
    pub nb_credit: usize,
}
