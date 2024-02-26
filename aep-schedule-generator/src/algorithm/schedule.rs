use super::{
    scores::{EvaluationOption, Score},
    taken_course::TakenCourse,
    taken_courses::TakenCourses,
};
use crate::data::{
    course::Course,
    time::{period::Period, weeks::Weeks},
};
use std::cmp::Ordering;

#[derive(PartialEq, Debug, Clone)]
pub struct Schedule<'a> {
    pub score: Score,
    pub week: Weeks,
    pub conflicts: u8,
    pub taken_courses: TakenCourses,
    pub courses: &'a [Course],
}

impl<'a> PartialOrd for Schedule<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

/// Trust me bro (I expect the evaluation function to not have stupid NaN value)
impl<'a> Eq for Schedule<'a> {
    fn assert_receiver_is_total_eq(&self) {}
}

impl<'a> Ord for Schedule<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.partial_cmp(&other.score).unwrap()
    }
}

impl<'a> Schedule<'a> {
    pub fn new(courses: &'a [Course]) -> Self {
        Self {
            score: Score::default(),
            week: Weeks::default(),
            conflicts: 0,
            taken_courses: TakenCourses::default(),
            courses,
        }
    }
    pub fn add(mut self, course: TakenCourse) -> Self {
        if let Some(theo_group) = &course.get_theo_group(self.courses) {
            for period in &theo_group.periods {
                self.week.add_period(period);
            }
        }
        if let Some(lab_group) = &course.get_lab_group(self.courses) {
            for period in &lab_group.periods {
                self.week.add_period(period);
            }
        }
        self.taken_courses.push(course);
        self
    }
    pub fn add_check_conflicts(
        &self,
        n: u8,
        min: f64,
        options: EvaluationOption,
        new_course: TakenCourse,
    ) -> Option<Self> {
        let mut new_schedule = self.clone();
        if let Some(theo_group) = new_course.get_theo_group(self.courses) {
            for period in &theo_group.periods {
                if new_schedule.week.conflict_in_day(period) {
                    new_schedule.conflicts += 1;
                    if new_schedule.conflicts > n {
                        return None;
                    }
                }
                new_schedule.add_update_score(period);
            }
        }
        if let Some(lab_group) = new_course.get_lab_group(self.courses) {
            for period in &lab_group.periods {
                if new_schedule.week.conflict_in_day(period) {
                    new_schedule.conflicts += 1;
                    if new_schedule.conflicts > n {
                        return None;
                    }
                }
                new_schedule.add_update_score(period)
            }
        }
        if new_schedule.score.evaluate(options) < min {
            return None;
        }
        new_schedule.taken_courses.push(new_course);
        Some(new_schedule)
    }
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
    // pub fn generate_ics(&self, calendar: &Calendar) -> String {
    //     let mut cal = IcalCalendarBuilder::version("2.0")
    //         .gregorian()
    //         .prodid("-//ical-rs//github.com//")
    //         .build();

    //     for course in self.taken_courses.iter().map(|c| c.get_course(courses)) {
    //         if let Some(lab) = &course.lab_group {
    //             for p in lab.periods.iter() {
    //                 calendar.iter_apply(p.week_nb, p.day, |d| {
    //                     let start =
    //                         date_to_timestamp(&d, p.hours.starting_hour(), p.hours.start_minutes());
    //                     let end =
    //                         date_to_timestamp(&d, p.hours.last_hour(), p.hours.last_minutes());
    //                     let event = IcalEventBuilder::tzid("America/New_York")
    //                         .uid(Uuid::new_v4())
    //                         .changed(chrono::Local::now().format("%Y%m%dT%H%M%S").to_string())
    //                         .start(start)
    //                         .end(end)
    //                         .set(ical_property!(
    //                             "SUMMARY",
    //                             format!("Laboratoire {}", course.sigle)
    //                         ))
    //                         .set(ical_property!("DESCRIPTION", p.room.to_string()))
    //                         .build();
    //                     cal.events.push(event);
    //                 });
    //             }
    //         }
    //         if let Some(theo) = &course.theo_group {
    //             for p in theo.periods.iter() {
    //                 calendar.iter_apply(p.week_nb, p.day, |d| {
    //                     let start =
    //                         date_to_timestamp(&d, p.hours.starting_hour(), p.hours.start_minutes());
    //                     let end =
    //                         date_to_timestamp(&d, p.hours.last_hour(), p.hours.last_minutes());
    //                     let event = IcalEventBuilder::tzid("America/New_York")
    //                         .uid(Uuid::new_v4())
    //                         .changed(chrono::Local::now().format("%Y%m%dT%H%M%S").to_string())
    //                         .start(start)
    //                         .end(end)
    //                         .set(ical_property!(
    //                             "SUMMARY",
    //                             format!("Théorie {}", course.sigle)
    //                         ))
    //                         .set(ical_property!("DESCRIPTION", p.room.to_string()))
    //                         .build();
    //                     cal.events.push(event);
    //                 });
    //             }
    //         }
    //     }
    //     cal.generate()
    // }
}
