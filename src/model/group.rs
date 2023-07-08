use super::period::Period;

#[derive(Debug, Clone)]
pub struct Group {
    name: String,
    nb_students: usize,
    students_capacity: usize,
    periods: Vec<Period>,
}
