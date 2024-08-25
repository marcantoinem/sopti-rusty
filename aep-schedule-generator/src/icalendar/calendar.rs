use std::{array, io::BufRead};

use serde::{Deserialize, Serialize};

use crate::{
    algorithm::schedule::Schedule,
    data::time::{day::Day, week_number::WeekNumber},
};

use super::{dates::Dates, dates_builder::DatesBuilder};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Calendar {
    weeks: [[Dates; 7]; 2],
    both: [Dates; 7],
}

impl Calendar {
    pub fn from_csv(days: impl BufRead) -> Self {
        let mut days = days.lines();
        let mut weeks: [[DatesBuilder; 7]; 2] = Default::default();
        let mut both: [DatesBuilder; 7] = Default::default();

        let mut session_start = String::new();
        let mut session_end = String::new();
        days.next();
        for day in days {
            let day = &day.unwrap();
            let mut day = day.split(",");
            let [Some(date), Some(day), Some(b_type)] = array::from_fn(|_| day.next()) else {
                continue;
            };
            if session_start.is_empty() {
                session_start = date.to_string();
            }
            session_end = date.to_string();
            let week: WeekNumber = b_type.into();
            let day: Day = day[0..3].into();

            weeks[week as usize][day as usize].add_date(date);
            both[day as usize].add_date(date);
        }

        let weeks = weeks.map(|week| {
            let mut i = 0;
            week.map(|d| {
                let d = d.build(&session_start, &session_end, (i as u8).into());
                i += 1;
                d
            })
        });

        let mut i = 0;

        let both = both.map(|d| {
            let d = d.build(&session_start, &session_end, (i as u8).into());
            i += 1;
            d
        });
        Self { weeks, both }
    }

    pub fn generate_ics(&self, schedule: &Schedule) -> String {
        let mut cal = icalendar::Calendar::new();
        cal.name("horaire");

        for course in schedule.taken_courses.iter() {
            course.for_each_group(|g, group_type| {
                for p in g.periods.iter() {
                    match p.week_nb {
                        WeekNumber::B1 | WeekNumber::B2 => self.weeks[p.week_nb as usize]
                            [p.day as usize]
                            .push_events(&mut cal, course, p, group_type),
                        WeekNumber::Both => {
                            self.both[p.day as usize].push_events(&mut cal, course, p, group_type)
                        }
                    }
                }
            });
        }
        cal.done().to_string()
    }
}
