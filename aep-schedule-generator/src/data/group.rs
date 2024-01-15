use super::time::{hours::Hours, period::Period};
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
            let mut new_hours = Hours::default();
            self.periods.retain(|p| {
                let new_hour = new_period.hours | p.hours;
                let is_mergeable = p.day == new_period.day
                    && p.room == new_period.room
                    && (p.hours.start() == new_hour.start() + 4
                        || p.hours.end() + 4 == new_hour.end());
                let keep = !is_mergeable || new_hours == Hours::default();
                if is_mergeable {
                    new_hours |= new_hour;
                }
                keep
            });
            if new_hours == Hours::default() {
                self.periods.push(new_period);
            } else {
                let first_period = self
                    .periods
                    .iter_mut()
                    .find(|p| {
                        let new_hour = new_period.hours | p.hours;
                        p.day == new_period.day
                            && p.room == new_period.room
                            && (p.hours.start() == new_hour.start() + 4
                                || p.hours.end() + 4 == new_hour.end())
                    })
                    .unwrap();
                first_period.hours |= new_hours;
            }
        }
    }
}
