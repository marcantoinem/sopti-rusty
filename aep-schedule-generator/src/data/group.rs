use super::time::period::Period;
use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Group {
    pub number: u8,
    pub open: bool,
    pub conflict: bool,
    pub periods: SmallVec<[Period; 2]>,
}

impl Group {
    pub fn new(number: &str, period: Period) -> Self {
        let number = number.parse().unwrap_or(0);
        Self {
            number,
            open: true,
            conflict: false,
            periods: smallvec![period],
        }
    }

    pub fn add_period(&mut self, new_group: Group) {
        for new_period in new_group.periods {
            if let Some(period) = self.periods.iter_mut().find(|p| {
                let new_hour = new_period.hours | p.hours;
                p.day == new_period.day
                    && p.room == new_period.room
                    && (p.hours.start_minutes() + 4 == new_hour.start_minutes()
                        || p.hours.last_minutes() + 4 == new_hour.last_minutes())
            }) {
                period.hours |= new_period.hours;
            } else {
                self.periods.push(new_period);
            }
        }
    }
}
