use std::{fs::File, io::{BufReader, BufRead}, array};
use aep_schedule_generator::data::time::day::Day;

fn main() {
    let days = BufReader::new(File::open("alternance.csv").unwrap());
    let mut days = days.lines();
    days.next();
     
    for day in days {
        let day = &day.unwrap();
        let mut day = day.split(",");
        let [Some(date), Some(day), Some(b_type)] = array::from_fn(|_| day.next())
            else {continue};
        let day: Day = day[0..3].into();
        println!("{}-{}-{}", date, day, b_type); 
    }
}
