use super::{
    hours::{Hours, NO_HOUR},
    period::Period,
};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Week([Hours; 7]);

impl Week {
    pub fn add_period(&mut self, period: &Period) {
        self.0[period.day as usize] |= period.hours;
    }
    pub fn conflict_in_day(&self, period: &Period) -> bool {
        self.0[period.day as usize] & period.hours != NO_HOUR
    }
}
