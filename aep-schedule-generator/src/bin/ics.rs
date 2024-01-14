use std::{fs::File, io::{BufReader, BufRead}, array};
use aep_schedule_generator::{data::{time::{day::Day, week_number::WeekNumber}, course::Course, courses::Courses}, algorithm::{scores::EvaluationOption, generation::SchedulesOptions}};
use ical::{generator::{IcalCalendarBuilder, IcalEventBuilder, Emitter}, ical_property};
use ical::property::Property;
use uuid::Uuid;

type Date = String;

#[derive(Default)]
struct Calendar {
    weeks: [[Vec<Date>; 7]; 2],
}

impl Calendar {
    /// There should be no day that are both B1 and B2 in the calendar
    pub fn push(&mut self, date: Date, week: WeekNumber, day: Day) {
        self.weeks[week as usize][day as usize].push(date);
    }
    /// A period can have Both weeks
    pub fn iter_apply(&self, week: WeekNumber, day: Day, mut function: impl FnMut(&Date)) {
        if week == WeekNumber::Both {
            for date in self.weeks[0][day as usize].iter().chain(self.weeks[1][day as usize].iter()) {
                function(date);
            }
            return;
        }
        for date in self.weeks[week as usize][day as usize].iter() {
            function(date);
        }
    }
}

fn date_to_timestamp(date: &str, hours: u8, minutes: u8) -> String {
    let mut timestamp: String = date.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
    timestamp.push_str(&format!("T{:0>2}{:0>2}00", hours, minutes));
    timestamp
}

fn main() {
    let horsage = BufReader::new(File::open("horsage.csv").unwrap());
    let fermes = BufReader::new(File::open("fermes.csv").unwrap());
    let courses = Courses::from_csv(horsage, fermes);
    let courses_to_take = vec![
        "INF2705", "LOG2990", "MTH2302D", "SSH3201", "SSH3501D",
    ];
    let mut courses_to_take: Vec<Course> = courses_to_take
        .into_iter()
        .map(|sigle| courses.get_course(sigle).unwrap())
        .collect();
    for course in courses_to_take.iter_mut() {
        for g in course.lab_groups.0.iter_mut() {
            g.open = true;
        }
        for g in course.theo_groups.0.iter_mut() {
            g.open = true;
        }
    }

    let evaluation = EvaluationOption {
        day_off: 2,
        morning: 2,
        finish_early: 2,
    };
    let options = SchedulesOptions {
        courses_to_take,
        max_nb_conflicts: 0,
        evaluation,
    };

    let result = options.get_schedules().into_sorted_vec();
    let result = result.first().unwrap();
    println!("{}", result);

    let days = BufReader::new(File::open("alternance.csv").unwrap());
    let mut days = days.lines();
    days.next();

    let mut cal = IcalCalendarBuilder::version("2.0")
        .gregorian()
        .prodid("-//ical-rs//github.com//")
        .build();
    
    let mut calendar = Calendar::default();

    for day in days {
        let day = &day.unwrap();
        let mut day = day.split(",");
        let [Some(date), Some(day), Some(b_type)] = array::from_fn(|_| day.next())
            else {continue};
        let week: WeekNumber = b_type.into();
        let day: Day = day[0..3].into();
        calendar.push(date.to_string(), week, day);
    }

    for course in &result.courses {
        if let Some(lab) = &course.lab_group {
            for p in lab.periods.iter() {
                calendar.iter_apply(p.week_nb, p.day, |d| {
                    let start = date_to_timestamp(&d, p.hours.starting_hour(), p.hours.start_minutes());
                    let end = date_to_timestamp(&d, p.hours.last_hour(), p.hours.last_minutes());
                    let event = IcalEventBuilder::tzid("America/New_York")
                        .uid(Uuid::new_v4())
                        .changed(chrono::Local::now().format("%Y%m%dT%H%M%S").to_string())
                        .start(start)
                        .end(end)
                        .set(ical_property!("SUMMARY", format!("{} - L", course.sigle)))
                        .build();
                    cal.events.push(event);
                });
            }
        }
        if let Some(theo) = &course.theo_group {
            for p in theo.periods.iter() {
                calendar.iter_apply(p.week_nb, p.day, |d| {
                    let start = date_to_timestamp(&d, p.hours.starting_hour(), p.hours.start_minutes());
                    let end = date_to_timestamp(&d, p.hours.last_hour(), p.hours.last_minutes());
                    let event = IcalEventBuilder::tzid("America/New_York")
                        .uid(Uuid::new_v4())
                        .changed(chrono::Local::now().format("%Y%m%dT%H%M%S").to_string())
                        .start(start)
                        .end(end)
                        .set(ical_property!("SUMMARY", format!("{} - T", course.sigle)))
                        .build();
                    cal.events.push(event);
                });
            }
        }
    }

    let _ = std::fs::write("test.ics", cal.generate());
}
