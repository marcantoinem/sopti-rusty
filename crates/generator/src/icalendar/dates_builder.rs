use chrono::{Datelike, NaiveDate};

use crate::data::time::day::Day;

use super::dates::Dates;

#[derive(Default)]
pub struct DatesBuilder {
    pub dates: Vec<NaiveDate>,
}

impl DatesBuilder {
    // Always add date in chronological order
    pub fn add_date(&mut self, date: &str) {
        let date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap();

        self.dates.push(date);
    }

    fn new_weekend(session_start: &str, session_end: &str, day: Day) -> Dates {
        let session_start = find_next_weekday(&session_start, day);
        let session_end = find_last_weekday(&session_end, day);
        Dates::Weekend {
            session_start,
            session_end,
        }
    }

    pub fn build(self, session_start: &str, session_end: &str, day: Day) -> Dates {
        let is_empty = self.dates.is_empty();
        match is_empty {
            false => Dates::Week(self.dates),
            _ => Self::new_weekend(session_start, session_end, day),
        }
    }
}

fn find_next_weekday(first_date: &str, next_day: Day) -> NaiveDate {
    let first_date = NaiveDate::parse_from_str(first_date, "%Y-%m-%d").unwrap();
    let mut date = next_day as i8 - first_date.weekday() as i8;
    if date < 0 {
        date += 7;
    }
    let fixed = first_date
        .checked_add_days(chrono::Days::new(date as u64))
        .unwrap();
    fixed
}

fn find_last_weekday(first_date: &str, next_day: Day) -> NaiveDate {
    let first_date = NaiveDate::parse_from_str(first_date, "%Y-%m-%d").unwrap();
    let mut date = next_day as i8 - first_date.weekday() as i8;
    if date > 0 {
        date -= 7;
    }
    let fixed = first_date
        .checked_sub_days(chrono::Days::new(-date as u64))
        .unwrap();
    fixed
}
