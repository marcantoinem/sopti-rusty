use aep_schedule_generator::{algorithm::generation::SchedulesOptions, data::courses::Courses};
use std::{fs::File, io::BufReader};

fn main() {
    let horsage = BufReader::new(File::open("horsage.csv").unwrap());
    let fermes = BufReader::new(File::open("fermes.csv").unwrap());
    let courses = Courses::from_csv(horsage, fermes);
    let inf1900 = courses.get_course("INF1900").unwrap();
    let options = SchedulesOptions {
        courses_to_take: vec![inf1900],
    };
    println!("{:#?}", options.get_schedules().into_sorted_vec());
}
