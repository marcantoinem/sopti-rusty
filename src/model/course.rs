enum CourseType {
    Lab,
    Theo,
}

pub struct Course {
    sigle: String,
    group: String,
    course_type: CourseType,
    nb_students: usize,
    capacity: usize,
}
