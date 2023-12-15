use super::{
    hours::{Hours, NO_HOUR},
    period::Period,
    week_number::WeekNumber,
};
use crate::data::time::week::Week;
use serde::{Deserialize, Serialize};

#[derive(Default, Deserialize, Serialize, PartialEq, Debug, Clone)]
pub struct Weeks([Week; 2]);

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
    pub fn iter(&self) -> impl Iterator<Item = Hours> + '_ {
        self.0.iter().map(|w| w.iter().cloned()).flatten()
    }
}
