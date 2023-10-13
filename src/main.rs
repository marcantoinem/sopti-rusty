#![allow(dead_code)]
mod course;
mod courses;
mod group;
mod period;
mod schedule;
mod schedule_course;
mod schedule_value;

use courses::Courses;
use std::{fs::File, io::BufReader};

use crate::schedule::Schedule;

fn main() {
    let csv_horsage = BufReader::new(File::open("horsage.csv").unwrap());
    let csv_fermes = BufReader::new(File::open("fermes.csv").unwrap());
    let courses = Courses::from_csv(csv_horsage, csv_fermes);
    println!(
        "{:#?}",
        courses.get_schedules(&["INF1600", "INF1015"], Schedule::allow_conflicts)
    )
}
