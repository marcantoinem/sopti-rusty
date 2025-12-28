use crate::{
    algorithm::taken_course::TakenCourse,
    data::time::{period::PeriodCourse, weeks::Weeks},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Conflicts(Vec<Conflict>);

#[derive(Debug, PartialEq, Clone)]
pub struct Conflict {
    weeks: Weeks,
    courses: Vec<PeriodCourse>,
}

impl Conflicts {
    pub fn new(weeks: &Weeks, courses: &mut [TakenCourse]) -> Self {
        let mut conflicts: Vec<Conflict> = Vec::with_capacity(courses.len());
        for course in courses.iter_mut() {
            let sigle = course.sigle.clone();
            let (group, _) = course.get_group_mut();
            for period in &group.periods {
                if weeks.conflict_in_day(period) {
                    let mut new_weeks = Weeks::default();
                    new_weeks.add_period(period);
                    let mut new_conflict = Conflict {
                        weeks: new_weeks,
                        courses: vec![PeriodCourse::new(period, sigle.clone())],
                    };
                    conflicts.retain_mut(|conflict| {
                        if conflict.weeks.conflict_in_day(period) {
                            new_conflict.weeks |= conflict.weeks.clone();
                            new_conflict.courses.append(&mut conflict.courses);
                            false
                        } else {
                            true
                        }
                    });
                    conflicts.push(new_conflict);
                }
            }
        }

        Self(conflicts)
    }
}
