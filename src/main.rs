mod algorithm;
mod data;

use crate::data::courses::Courses;
use crate::{algorithm::schedule::Schedule, data::courses::COUNTER};
use std::{fs::File, io::BufReader, time::Instant};

fn main() {
    let csv_horsage = BufReader::new(File::open("horsage.csv").unwrap());
    let csv_fermes = BufReader::new(File::open("fermes.csv").unwrap());
    let courses = Courses::from_csv(csv_horsage, csv_fermes);
    let instant = Instant::now();
    let schedules = courses.get_schedules(
        &[
            "INF1500", "MTH1007", "INF1007", "INF1040", "MTH1101", "MTH2302D",
        ],
        Schedule::allow_conflicts,
        Schedule::more_day_off,
        50,
    );
    unsafe {
        println!(
            "{} {}s {} combinations",
            schedules.into_sorted_vec().first().unwrap(),
            instant.elapsed().as_secs_f64(),
            COUNTER
        );
    }
}
