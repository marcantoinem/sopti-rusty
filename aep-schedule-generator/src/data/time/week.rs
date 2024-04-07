use super::{
    hours::{Hours, NO_HOUR},
    period::Period,
};
use std::ops::Deref;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Week<const N: usize>([Hours; N]);

impl<const N: usize> Default for Week<N> {
    fn default() -> Self {
        Self([NO_HOUR; N])
    }
}

impl<const N: usize> Deref for Week<N> {
    type Target = [Hours; N];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> Week<N> {
    pub fn new(week: [u64; N]) -> Self {
        Self(week.map(|d| Hours::from(d)))
    }
    pub fn add_period(&mut self, period: &Period) {
        self.0[period.day as usize] |= period.hours;
    }
    pub fn conflict_in_day(&self, period: &Period) -> bool {
        self.0[period.day as usize] & period.hours != NO_HOUR
    }
    pub fn user_conflict_in_day(&self, period: &Period) -> bool {
        if period.day as usize >= N {
            return false;
        }
        self.0[period.day as usize] & period.hours != NO_HOUR
    }
}
