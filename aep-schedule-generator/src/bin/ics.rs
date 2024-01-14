use aep_schedule_generator::{
    algorithm::{generation::SchedulesOptions, scores::EvaluationOption},
    data::{course::Course, courses::Courses, time::calendar::Calendar},
};
use std::{fs::File, io::BufReader};

fn main() {
    let horsage = BufReader::new(File::open("horsage.csv").unwrap());
    let fermes = BufReader::new(File::open("fermes.csv").unwrap());
    let courses = Courses::from_csv(horsage, fermes);
    let courses_to_take = vec!["INF2705", "LOG2990", "MTH2302D", "SSH3201", "SSH3501D"];
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
    let calendar = Calendar::from_csv(days);

    let _ = std::fs::write("test.ics", result.generate_ics(&calendar));
}
