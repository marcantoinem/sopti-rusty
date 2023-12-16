use super::{scores::Score, taken_course::TakenCourse};
use crate::data::time::{
    hours::{Hours, NO_HOUR},
    weeks::Weeks,
};
use serde::{Deserialize, Serialize};
use std::{cmp::Ordering, fmt::Display};

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
    pub fn add_check_conflicts(&self, n: u8, new_course: &TakenCourse) -> Option<Schedule> {
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
                new_schedule.week.add_period(period);
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
                new_schedule.week.add_period(period);
            }
        }
        new_schedule.courses.push(new_course);
        Some(new_schedule)
    }
    /// This check course up to 13h
    pub fn more_morning(&self) -> f64 {
        const BEST_MORNING: u32 = 4 * 5;
        let mut sum = 0;
        let mut non_zero_day = 0;
        let mut min = u32::MAX;
        for day in self.week.iter() {
            if day != NO_HOUR {
                let morning_hours = day.trailing_zeros();
                non_zero_day += 1;
                sum += morning_hours;
                min = std::cmp::min(min, morning_hours);
            }
        }
        let average = sum as f64 / (non_zero_day * BEST_MORNING) as f64;
        let min = min as f64 / BEST_MORNING as f64;
        0.5 * average + 0.5 * min
    }
    pub fn day_off(&self) -> f64 {
        self.week
            .iter()
            .map(|d| if d == NO_HOUR { 1.0 } else { 0.0 })
            .sum::<f64>()
            / 12.0
    }

    pub fn finish_early(&self) -> f64 {
        const BEST_AFTERNOON: u32 = 64 - 4 * 5;
        let mut sum = 0;
        let mut non_zero_day = 0;
        let mut min = u32::MAX;
        for day in self.week.iter() {
            if day != NO_HOUR {
                let afternoon_hours = day.leading_zeros();
                non_zero_day += 1;
                sum += afternoon_hours;
                min = std::cmp::min(min, afternoon_hours);
            }
        }
        let average = sum as f64 / (non_zero_day * BEST_AFTERNOON) as f64;
        let min = min as f64 / BEST_AFTERNOON as f64;
        0.5 * average + 0.5 * min
    }
}
