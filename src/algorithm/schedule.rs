use super::taken_course::TakenCourse;
use crate::data::time::{
    hours::{Hours, NO_HOUR},
    week::Week,
};
use std::{cmp::Ordering, fmt::Display};

#[derive(PartialEq, Debug, Clone)]
pub struct Schedule {
    pub value: f64,
    week: Week,
    courses: Vec<TakenCourse>,
}

impl Default for Schedule {
    fn default() -> Self {
        Self {
            value: 0.0,
            week: Week::default(),
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
        self.value.partial_cmp(&other.value)
    }
}

/// Trust me bro (I expect the evaluation function to not have stupid NaN value)
impl Eq for Schedule {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Ord for Schedule {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.partial_cmp(&other.value).unwrap()
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
    pub fn allow_conflicts(&self, _new_course: &TakenCourse) -> bool {
        true
    }
    /// This is just a nice functor, it did nothing wrong!
    pub fn allow_n_conflicts(n: u8) -> impl Fn(&Schedule, &TakenCourse) -> bool {
        move |schedule, new_course| {
            let mut conflicts = 0;
            let mut new_week = schedule.week.clone();
            if let Some(theo_group) = &new_course.theo_group {
                for period in &theo_group.periods {
                    if new_week.conflict_in_day(period) {
                        conflicts += 1;
                        if conflicts > n {
                            return false;
                        }
                    }
                    new_week.add_period(period);
                }
            }
            if let Some(lab_group) = &new_course.lab_group {
                for period in &lab_group.periods {
                    if new_week.conflict_in_day(period) {
                        conflicts += 1;
                        if conflicts > n {
                            return false;
                        }
                    }
                    new_week.add_period(period);
                }
            }
            true
        }
    }
    pub fn forbid_conflicts(&self, new_course: &TakenCourse) -> bool {
        let mut new_week = self.week.clone();
        if let Some(theo_group) = &new_course.theo_group {
            for period in &theo_group.periods {
                if new_week.conflict_in_day(period) {
                    return false;
                }
                new_week.add_period(period);
            }
        }
        if let Some(lab_group) = &new_course.lab_group {
            for period in &lab_group.periods {
                if new_week.conflict_in_day(period) {
                    return false;
                }
                new_week.add_period(period);
            }
        }
        true
    }
    pub fn more_morning(&self) -> f64 {
        let mut sum = 0;
        let mut non_zero_day = 0;
        for day in 0..7 {
            if self.week[day] != NO_HOUR {
                non_zero_day += 1;
                sum += self.week[day].trailing_zeros();
            }
        }
        sum as f64 / non_zero_day as f64
    }
    pub fn more_day_off(&self) -> f64 {
        self.week
            .iter()
            .map(|d| if d == &NO_HOUR { 1.0 } else { 0.0 })
            .sum()
    }
    pub fn finish_early(&self) -> f64 {
        let mut sum = 0;
        let mut non_zero_day = 0;
        for day in 0..7 {
            if self.week[day] != NO_HOUR {
                non_zero_day += 1;
                sum += self.week[day].leading_zeros();
            }
        }
        sum as f64 / non_zero_day as f64
    }
}
