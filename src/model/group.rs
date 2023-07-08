use super::period::Period;

#[derive(Debug, Clone)]
pub struct Group {
    pub name: String,
    pub nb_students: usize,
    pub students_capacity: usize,
    pub periods: Vec<Period>,
}
