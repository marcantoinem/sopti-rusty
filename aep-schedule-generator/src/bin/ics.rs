use aep_schedule_generator::{
    algorithm::{generation::SchedulesOptions, scores::EvaluationOption},
    data::{course::Course, courses::Courses, time::week::Week},
    icalendar::calendar::Calendar,
};
use std::{fs::File, io::BufReader};

fn main() {
    let horsage = BufReader::new(File::open("horsage.csv").unwrap());
    let fermes = BufReader::new(File::open("fermes.csv").unwrap());
    let courses = Courses::from_csv(horsage, fermes);
    let courses_to_take = vec!["INF2705"];
    let mut courses_to_take: Vec<Course> = courses_to_take
        .into_iter()
        .map(|sigle| courses.get_course(sigle).unwrap())
        .collect();
    for course in courses_to_take.iter_mut() {
        course.for_each_groups_mut(|g| g.open = true);
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
        user_conflicts: Week::default(),
        max_size: 69,
    };

    let result = options.get_schedules().into_sorted_vec();
    let result = result.first().unwrap();

    let days = BufReader::new(File::open("alternance.csv").unwrap());
    let calendar = Calendar::from_csv(days);

    println!("{:#?}", calendar);

    let _ = std::fs::write("test.ics", calendar.generate_ics(&result));
}
