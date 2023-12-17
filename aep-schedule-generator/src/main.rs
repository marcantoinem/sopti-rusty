use aep_schedule_generator::{
    algorithm::{generation::SchedulesOptions, scores::EvaluationOption},
    data::courses::Courses,
};
use std::{fs::File, io::BufReader, time::Instant};

fn main() {
    let horsage = BufReader::new(File::open("horsage.csv").unwrap());
    let fermes = BufReader::new(File::open("fermes.csv").unwrap());
    let courses = Courses::from_csv(horsage, fermes);
    let courses_to_take = vec![
        "INF1900", "LOG1810", "INF1015", "INF1600", "SSH3201", "INF3405",
    ];
    let mut courses_to_take: Vec<_> = courses_to_take
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
    let instant = Instant::now();
    let result = options.get_schedules().into_sorted_vec();

    println!(
        "{:#?} computed in {:?}",
        result,
        instant.elapsed().as_secs_f64()
    );
}
