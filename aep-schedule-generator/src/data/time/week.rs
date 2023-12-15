use super::{
    hours::{Hours, NO_HOUR},
    period::Period,
};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone, Copy, Default)]
pub struct Week([Hours; 7]);

impl Deref for Week {
    type Target = [Hours; 7];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Week {
    pub fn add_period(&mut self, period: &Period) {
        self.0[period.day as usize] |= period.hours;
    }
    pub fn conflict_in_day(&self, period: &Period) -> bool {
        self.0[period.day as usize] & period.hours != NO_HOUR
    }
}
