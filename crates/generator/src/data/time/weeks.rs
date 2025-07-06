use super::{
    day::Day,
    hours::{Hours, NO_HOUR},
    period::Period,
    week_number::WeekNumber,
};
use crate::{
    algorithm::scores::{BEST_AFTERNOON, BEST_MORNING},
    data::time::week::Week,
};
use std::cmp;

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Weeks([Week<7>; 2]);

impl Weeks {
    pub fn add_period(&mut self, period: &Period) {
        if period.week_nb == WeekNumber::Both {
            self.0[0].add_period(period);
            self.0[1].add_period(period);
            return;
        }
        self.0[period.week_nb as usize].add_period(period);
    }
    pub fn conflict_in_day(&self, period: &Period) -> bool {
        if period.week_nb == WeekNumber::Both {
            return (self.0[0][period.day as usize] | self.0[1][period.day as usize])
                & period.hours
                != NO_HOUR;
        }
        self.0[period.week_nb as usize][period.day as usize] & period.hours != NO_HOUR
    }

    pub fn hours(&self, day: Day) -> [Hours; 2] {
        [self.0[0][day as usize], self.0[1][day as usize]]
    }

    pub fn get_day_off(&self, period: &Period) -> u8 {
        self.hours(period.day)
            .into_iter()
            .map(|h| (h == NO_HOUR) as u8)
            .sum()
    }
    pub fn get_morning(&self, period: &Period) -> u16 {
        self.hours(period.day)
            .into_iter()
            .filter(|h| *h != NO_HOUR)
            .map(|h| cmp::min(BEST_MORNING as u16, h.trailing_zeros() as u16))
            .sum()
    }
    pub fn get_finish_early(&self, period: &Period) -> u16 {
        self.hours(period.day)
            .into_iter()
            .filter(|h| *h != NO_HOUR)
            .map(|h| cmp::min(BEST_AFTERNOON as u16, h.leading_zeros() as u16))
            .sum()
    }
    pub fn iter(&self) -> impl Iterator<Item = Hours> + '_ {
        self.0.iter().map(|w| w.iter().cloned()).flatten()
    }
}
