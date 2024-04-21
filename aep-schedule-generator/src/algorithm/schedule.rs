use super::{
    scores::{EvaluationOption, Score},
    taken_course::{TakenCourse, TakenCourseBuilder},
    taken_courses::TakenCourses,
};
use crate::data::{
    course::Course,
    time::{
        calendar::{date_to_timestamp, Calendar},
        day::Day,
        hours::NO_HOUR,
        period::Period,
        week::Week,
        weeks::Weeks,
    },
};
use ical::{
    generator::{Emitter, IcalCalendarBuilder, IcalEventBuilder},
    ical_property,
    property::Property,
};
use std::cmp::Ordering;
use uuid::Uuid;

#[derive(PartialEq, Debug, Clone)]
pub struct ScheduleBuilder<'a> {
    pub score: Score,
    pub week: Weeks,
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
    pub fn new(courses: &'a [Course]) -> Self {
        Self {
            score: Score::default(),
            week: Weeks::default(),
            conflicts: 0,
            taken_courses: TakenCourses::default(),
            courses,
        }
    }
    #[inline(always)]
    pub fn add(mut self, course: TakenCourseBuilder) -> Self {
        course.for_each_group(self.courses, |group| {
            for period in &group.periods {
                self.week.add_period(period);
            }
        });
        self.taken_courses.push(course);
        self
    }
    #[inline(always)]
    pub fn add_check_conflicts(
        &self,
        n: u8,
        min: f64,
        user_conflicts: &Week<5>,
        options: EvaluationOption,
        new_course: TakenCourseBuilder,
    ) -> Option<Self> {
        let mut new_schedule = self.clone();
        let mut is_cancelled = false;
        new_course.for_each_group(self.courses, |group| {
            for period in &group.periods {
                if user_conflicts.user_conflict_in_day(period) {
                    is_cancelled = true;
                    return;
                }
                if new_schedule.week.conflict_in_day(period) {
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
        let day_off = self.week.get_day_off(period);
        let morning_hours = self.week.get_morning(period);
        self.score.day_off -= day_off;
        self.score.morning_hours -= morning_hours;
        self.score.afternoon_hours -= self.week.get_finish_early(period);
        self.week.add_period(period);
        self.score.day_off += self.week.get_day_off(period);
        self.score.morning_hours += self.week.get_morning(period);
        self.score.afternoon_hours += self.week.get_finish_early(period);
    }

    pub fn build(self) -> Schedule {
        let last_day = if self
            .week
            .hours(Day::Sunday)
            .iter()
            .chain(self.week.hours(Day::Saturday).iter())
            .all(|d| *d == NO_HOUR)
        {
            5
        } else {
            7
        };
        let taken_courses = self
            .taken_courses
            .iter()
            .map(|c| c.build(self.courses))
            .collect();
        Schedule {
            taken_courses,
            last_day,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Schedule {
    pub taken_courses: Vec<TakenCourse>,
    pub last_day: u8,
}

impl Schedule {
    pub fn generate_ics(&self, calendar: &Calendar) -> String {
        let mut cal = IcalCalendarBuilder::version("2.0")
            .gregorian()
            .prodid("-//ical-rs//github.com//")
            .build();

        for course in self.taken_courses.iter() {
            course.for_each_group(|g| {
                for p in g.periods.iter() {
                    calendar.iter_apply(p.week_nb, p.day, |d| {
                        let start =
                            date_to_timestamp(&d, p.hours.starting_hour(), p.hours.start_minutes());
                        let end =
                            date_to_timestamp(&d, p.hours.last_hour(), p.hours.last_minutes());
                        let event = IcalEventBuilder::tzid("America/New_York")
                            .uid(Uuid::new_v4())
                            .changed(chrono::Local::now().format("%Y%m%dT%H%M%S").to_string())
                            .start(start)
                            .end(end)
                            .set(ical_property!(
                                "SUMMARY",
                                format!("Laboratoire {}", course.sigle)
                            ))
                            .set(ical_property!("DESCRIPTION", p.room.to_string()))
                            .build();
                        cal.events.push(event);
                    });
                }
            });
        }
        cal.generate()
    }
}
