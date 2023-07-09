#![allow(dead_code)]
use model::courses::Courses;
use std::{fs::File, io::BufReader};

use crate::model::schedule::Schedule;

mod model;

fn main() {
    let csv_horsage = BufReader::new(File::open("horsage.csv").unwrap());
    let csv_fermes = BufReader::new(File::open("fermes.csv").unwrap());
    let courses = Courses::from_csv(csv_horsage, csv_fermes);
    println!(
        "{:#?}",
        courses.get_schedules(&["INF1600", "INF1015"], Schedule::allow_conflicts)
    )
}
