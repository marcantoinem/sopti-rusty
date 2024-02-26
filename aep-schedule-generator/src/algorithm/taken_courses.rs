use super::taken_course::TakenCourse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TakenCourses {
    len: u8,
    taken_courses: [TakenCourse; 9],
}

impl TakenCourses {
    pub fn push(&mut self, taken_course: TakenCourse) {
        self.taken_courses[self.len as usize] = taken_course;
        self.len += 1;
    }
}
