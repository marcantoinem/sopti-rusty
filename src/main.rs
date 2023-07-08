#![allow(unused_variables, dead_code)]
use model::courses::Courses;
use std::{fs::File, io::BufReader};

mod model;

fn main() {
    let csv_courses = BufReader::new(File::open("courses.csv").unwrap());
    let csv_periods = BufReader::new(File::open("periods.csv").unwrap());
    let courses = Courses::from_csv(csv_courses, csv_periods);
    println!("{:#?}", courses);
}
