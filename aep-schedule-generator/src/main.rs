use aep_schedule_generator::{
    algorithm::{generation::SchedulesOptions, scores::EvaluationOption},
    data::{courses::Courses, time::week::Week},
};
use std::{fs::File, io::BufReader, time::Instant};

fn main() {
    let horsage = BufReader::new(File::open("horsage.csv").unwrap());
    let fermes = BufReader::new(File::open("fermes.csv").unwrap());
    let courses = Courses::from_csv(horsage, fermes);
    let courses_to_take = vec![
        "INF1900", "LOG1810", "INF1015", "INF1600", "PHS1101", "MTH1007", "MTH1101", "SSH3201",
        "SSH3501",
    ];
    let courses_to_take: Vec<_> = courses_to_take
        .into_iter()
        .map(|sigle| courses.get_course(sigle).unwrap())
        .collect();
    let evaluation = EvaluationOption {
        day_off: 2,
        morning: 2,
        finish_early: 2,
    };
    let options = SchedulesOptions {
        courses_to_take,
        max_nb_conflicts: 3,
        evaluation,
        max_size: 100,
        user_conflicts: Week::default(),
    };
    let instant = Instant::now();
    let schedules = options.get_schedules();
    let number = schedules.number;
    let result = schedules.into_sorted_vec();

    println!(
        "{:?} computed in {:?} {} combinations evaluated",
        result.len(),
        instant.elapsed().as_secs_f64(),
        number,
    );
}
