use chrono::{NaiveDate, NaiveDateTime};
use icalendar::{Calendar, Component, Event, EventLike};
use serde::{Deserialize, Serialize};

use crate::algorithm::taken_course::TakenCourse;
use crate::data::group_sigle::GroupType;
use crate::data::time::period::Period;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Dates {
    Week(Vec<NaiveDate>),
    Weekend {
        session_start: NaiveDate,
        session_end: NaiveDate,
    },
}

const NAIVE_DATE_TIME_FORMAT: &str = "%Y%m%dT%H%M%S";

impl Dates {
    pub fn push_events(
        &self,
        cal: &mut Calendar,
        course: &TakenCourse,
        p: &Period,
        group_type: GroupType,
    ) {
        let labo = match group_type {
            GroupType::LabGroup => "Laboratoire",
            GroupType::TheoGroup => "Théorie",
        };

        let mut main = Event::new();

        main.summary(&format!("{} {}", labo, course.sigle))
            .description(p.room.as_str())
            .location(p.room.as_str());

        match self {
            Dates::Week(all_dates) => {
                let session_start = all_dates[0];
                let start = session_start
                    .and_hms_opt(
                        p.hours.starting_hour() as u32,
                        p.hours.start_minutes() as u32,
                        0,
                    )
                    .unwrap();
                let end = session_start
                    .and_hms_opt(p.hours.last_hour() as u32, p.hours.last_minutes() as u32, 0)
                    .unwrap();

                main.starts(start).ends(end);

                if all_dates.len() > 1 {
                    let start_dates: Vec<NaiveDateTime> = all_dates
                        .into_iter()
                        .map(|d| {
                            d.and_hms_opt(
                                p.hours.starting_hour() as u32,
                                p.hours.start_minutes() as u32,
                                0,
                            )
                            .unwrap()
                        })
                        .collect();

                    let mut rdate = start_dates[0].format(NAIVE_DATE_TIME_FORMAT).to_string();
                    for date in start_dates[1..].iter() {
                        let date = date.format(NAIVE_DATE_TIME_FORMAT);
                        rdate.push(',');
                        rdate.push_str(&date.to_string());
                    }

                    main.add_property("RDATE", &rdate);
                }
            }
            Dates::Weekend {
                session_start,
                session_end,
            } => {
                let start = session_start
                    .and_hms_opt(
                        p.hours.starting_hour() as u32,
                        p.hours.start_minutes() as u32,
                        0,
                    )
                    .unwrap();
                let end = session_start
                    .and_hms_opt(p.hours.last_hour() as u32, p.hours.last_minutes() as u32, 0)
                    .unwrap();
                let last = session_end
                    .and_hms_opt(
                        p.hours.starting_hour() as u32,
                        p.hours.start_minutes() as u32,
                        0,
                    )
                    .unwrap()
                    .format(NAIVE_DATE_TIME_FORMAT);
                let rrule = format!("FREQ=WEEKLY;UNTIL={}", last);

                main.starts(start).ends(end).add_property("RRULE", &rrule);
            }
        }

        cal.push(main.done());
    }
}
