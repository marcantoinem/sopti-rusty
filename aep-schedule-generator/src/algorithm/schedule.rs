use super::{
    scores::{EvaluationOption, Score},
    taken_course::TakenCourse,
};
use crate::data::time::{
    calendar::{date_to_timestamp, Calendar},
    hours::{Hours, NO_HOUR},
    period::Period,
    weeks::Weeks,
};
use ical::{generator::IcalEventBuilder, ical_property};
use ical::{
    generator::{Emitter, IcalCalendarBuilder},
    property::Property,
};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Display};
use uuid::Uuid;

#[derive(Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Schedule {
    pub score: Score,
    pub week: Weeks,
    pub conflicts: u8,
    pub courses: Vec<TakenCourse>,
}

impl Default for Schedule {
    fn default() -> Self {
        Self {
            score: Score::default(),
            week: Weeks::default(),
            conflicts: 0,
            courses: Vec::with_capacity(8),
        }
    }
}

// This omits 15 min gap even if the data is taken in account
impl Display for Schedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut to_print = String::new();
        for c in &self.courses {
            to_print.push_str(&c.to_string());
        }
        to_print.push_str(
            "      |    lundi     |    mardi     |   mercredi   |    jeudi     |   vendredi   |\n",
        );
        let mut i = 0;
        for hour in 8..=21 {
            for min in 0..4 {
                if min % 2 == 1 {
                    i += 1;
                    continue;
                }
                let min = min * 15;
                let time = format!("{}h{:0>2}", hour, min);
                to_print.push_str(&format!("{: <6}|", time));

                for day in 0..5 {
                    let course = self.get_course(day, i);
                    to_print.push_str(&format!("{: ^14}|", course));
                }
                to_print.push('\n');
                i += 1;
            }
        }
        writeln!(f, "{}", to_print)
    }
}

impl PartialOrd for Schedule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

/// Trust me bro (I expect the evaluation function to not have stupid NaN value)
impl Eq for Schedule {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Ord for Schedule {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.partial_cmp(&other.score).unwrap()
    }
}

impl Schedule {
    // A bit expensive O(n) where n is the number of period,
    // only use this for displaying the schedule
    fn get_course(&self, day: u8, hour: u8) -> String {
        for course in &self.courses {
            let periods = [&course.lab_group, &course.theo_group];
            let periods = periods
                .iter()
                .filter_map(|x| x.as_ref().and_then(|g| Some(&g.periods)))
                .flatten();
            for period in periods {
                if period.day as u8 == day && Hours(1 << hour) & period.hours != NO_HOUR {
                    return course.sigle.to_string();
                }
            }
        }

        String::new()
    }
    pub fn add(mut self, course: TakenCourse) -> Schedule {
        if let Some(theo_group) = &course.theo_group {
            for period in &theo_group.periods {
                self.week.add_period(period);
            }
        }
        if let Some(lab_group) = &course.lab_group {
            for period in &lab_group.periods {
                self.week.add_period(period);
            }
        }
        self.courses.push(course);
        self
    }
    pub fn add_check_conflicts(
        &self,
        n: u8,
        min: f64,
        options: EvaluationOption,
        new_course: &TakenCourse,
    ) -> Option<Schedule> {
        let mut new_schedule = self.clone();
        let mut new_course = new_course.clone();
        if let Some(theo_group) = &mut new_course.theo_group {
            for period in &theo_group.periods {
                if new_schedule.week.conflict_in_day(period) {
                    new_schedule.conflicts += 1;
                    theo_group.conflict = true;
                    if new_schedule.conflicts > n {
                        return None;
                    }
                }
                new_schedule.add_update_score(period);
            }
        }
        if let Some(lab_group) = &mut new_course.lab_group {
            for period in &lab_group.periods {
                if new_schedule.week.conflict_in_day(period) {
                    new_schedule.conflicts += 1;
                    lab_group.conflict = true;
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
        new_schedule.courses.push(new_course);
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
    pub fn generate_ics(&self, calendar: &Calendar) -> String {
        let mut cal = IcalCalendarBuilder::version("2.0")
            .gregorian()
            .prodid("-//ical-rs//github.com//")
            .build();

        for course in &self.courses {
            if let Some(lab) = &course.lab_group {
                for p in lab.periods.iter() {
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
            }
            if let Some(theo) = &course.theo_group {
                for p in theo.periods.iter() {
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
                                format!("Théorie {}", course.sigle)
                            ))
                            .set(ical_property!("DESCRIPTION", p.room.to_string()))
                            .build();
                        cal.events.push(event);
                    });
                }
            }
        }
        cal.generate()
    }
}
