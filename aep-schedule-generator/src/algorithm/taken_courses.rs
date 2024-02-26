use super::taken_course::TakenCourse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TakenCourses {
    len: u8,
    taken_courses: [TakenCourse; 9],
}

impl TakenCourses {
    #[inline(always)]
    pub fn push(&mut self, taken_course: TakenCourse) {
        self.taken_courses[self.len as usize] = taken_course;
        self.len += 1;
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = TakenCourse> + '_ {
        self.taken_courses[0..self.len as usize]
            .into_iter()
            .copied()
    }
}
