use std::{array, io::BufRead};

use serde::{Deserialize, Serialize};

use super::{day::Day, week_number::WeekNumber};

type Date = String;

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Calendar {
    weeks: [[Vec<Date>; 7]; 2],
}

impl Calendar {
    pub fn from_csv(days: impl BufRead) -> Self {
        let mut days = days.lines();
        days.next();
        let mut calendar = Calendar::default();

        for day in days {
            let day = &day.unwrap();
            let mut day = day.split(",");
            let [Some(date), Some(day), Some(b_type)] = array::from_fn(|_| day.next()) else {
                continue;
            };
            let week: WeekNumber = b_type.into();
            let day: Day = day[0..3].into();
            calendar.push(date.to_string(), week, day);
        }
        calendar
    }
    /// There should be no day that are both B1 and B2 in the calendar
    pub fn push(&mut self, date: Date, week: WeekNumber, day: Day) {
        self.weeks[week as usize][day as usize].push(date);
    }
    pub fn iter_apply(&self, week: WeekNumber, day: Day, mut function: impl FnMut(&Date)) {
        if week == WeekNumber::Both {
            for date in self.weeks[0][day as usize]
                .iter()
                .chain(self.weeks[1][day as usize].iter())
            {
                function(date);
            }
            return;
        }
        for date in self.weeks[week as usize][day as usize].iter() {
            function(date);
        }
    }
}

pub fn date_to_timestamp(date: &str, hours: u8, minutes: u8) -> String {
    let mut timestamp: String = date.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    timestamp.push_str(&format!("T{:0>2}{:0>2}00", hours, minutes));
    timestamp
}
