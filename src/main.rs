mod algorithm;
mod data;

use crate::algorithm::schedule::Schedule;
use crate::data::courses::Courses;
use std::{fs::File, io::BufReader, time::Instant};

fn main() {
    let csv_horsage = BufReader::new(File::open("horsage.csv").unwrap());
    let csv_fermes = BufReader::new(File::open("fermes.csv").unwrap());
    let courses = Courses::from_csv(csv_horsage, csv_fermes);
    let instant = Instant::now();
    let schedules = courses.get_schedules(
        &["INF1900", "INF1015", "INF1600", "INF3500", "INF2610"],
        Schedule::forbid_conflicts,
        Schedule::more_morning,
        50,
    );
    println!("{:?} {}", schedules, instant.elapsed().as_secs_f64());
}
