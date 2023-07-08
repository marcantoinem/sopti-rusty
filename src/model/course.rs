use super::group::Group;

#[derive(Debug, Clone)]
pub struct Course {
    pub sigle: String,
    pub theo_group: Vec<Group>,
    pub lab_group: Vec<Group>,
    pub nb_credit: usize,
}
