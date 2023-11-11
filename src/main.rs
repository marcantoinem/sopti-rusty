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
        &["INF1500", "MTH1007", "INF1007"],
        Schedule::forbid_conflicts,
        Schedule::more_morning,
        1,
    );
    println!(
        "{} {}s",
        schedules.into_sorted_vec().first().unwrap(),
        instant.elapsed().as_secs_f64()
    );
}
