use super::{
    scores::{EvaluationOption, Score},
    taken_course::{TakenCourse, TakenCourseBuilder},
    taken_courses::TakenCourses,
};
use crate::{
    algorithm::conflicts::Conflicts,
    data::{
        course::Course,
        time::{hours::NO_HOUR, period::Period, weeks::Weeks},
    },
};
use std::{
    cmp::Ordering,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(PartialEq, Debug, Clone)]
pub struct ScheduleBuilder<'a> {
    pub score: Score,
    pub weeks: Weeks,
    pub conflicts: u8,
    pub taken_courses: TakenCourses,
    pub courses: &'a [Course],
}

impl<'a> PartialOrd for ScheduleBuilder<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

/// Trust me bro (I expect the evaluation function to not have stupid NaN value)
impl<'a> Eq for ScheduleBuilder<'a> {
    fn assert_receiver_is_total_eq(&self) {}
}

impl<'a> Ord for ScheduleBuilder<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.partial_cmp(&other.score).unwrap()
    }
}

impl<'a> ScheduleBuilder<'a> {
    #[inline(always)]
    pub(super) fn new(courses: &'a [Course]) -> Self {
        Self {
            score: Score::default(),
            weeks: Weeks::default(),
            conflicts: 0,
            taken_courses: TakenCourses::default(),
            courses,
        }
    }
    #[inline(always)]
    pub(super) fn add_check_conflicts(
        &self,
        n: u8,
        min: f64,
        options: EvaluationOption,
        new_course: TakenCourseBuilder,
    ) -> Option<Self> {
        let mut new_schedule = self.clone();
        let mut is_cancelled = false;
        new_course.for_each_group(self.courses, |group| {
            for period in group.periods.iter() {
                if new_schedule.weeks.conflict_in_day(period) {
                    new_schedule.conflicts += 1;
                    if new_schedule.conflicts > n {
                        is_cancelled = true;
                        return;
                    }
                }
                new_schedule.add_update_score(period);
            }
        });
        if is_cancelled || new_schedule.score.evaluate(options) < min {
            return None;
        }
        new_schedule.taken_courses.push(new_course);
        Some(new_schedule)
    }

    #[inline(always)]
    fn add_update_score(&mut self, period: &Period) {
        let day_off = self.weeks.get_day_off(period);
        let morning_hours = self.weeks.get_morning(period);
        self.score.day_off -= day_off;
        self.score.morning_hours -= morning_hours;
        self.score.afternoon_hours -= self.weeks.get_finish_early(period);
        self.weeks.add_period(period);
        self.score.day_off += self.weeks.get_day_off(period);
        self.score.morning_hours += self.weeks.get_morning(period);
        self.score.afternoon_hours += self.weeks.get_finish_early(period);
    }

    pub(super) fn build(self) -> Schedule {
        let last_day = if self.weeks.weekend().iter().all(|d| *d == NO_HOUR) {
            5
        } else {
            7
        };
        let mut courses: Vec<TakenCourse> = self
            .taken_courses
            .iter()
            .map(|c| c.build(self.courses))
            .collect();

        // Take a mutable reference to indicate which period are in conflict
        let conflicts = Conflicts::new(&self.weeks, &mut courses);

        let mut hasher = DefaultHasher::new();
        courses.hash(&mut hasher);
        let id = hasher.finish();

        Schedule {
            courses,
            conflicts,
            last_day,
            id,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Schedule {
    pub courses: Vec<TakenCourse>,
    pub conflicts: Conflicts,
    pub last_day: u8,
    pub id: u64,
}
