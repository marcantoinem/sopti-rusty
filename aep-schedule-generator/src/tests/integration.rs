use crate::{
    algorithm::{generation::SchedulesOptions, scores::EvaluationOption},
    data::{course::Course, courses::Courses},
};
use std::{fs::File, io::BufReader};

fn get_h24_courses() -> Courses {
    let horsage = BufReader::new(File::open("horsage.csv").unwrap());
    let fermes = BufReader::new(File::open("fermes.csv").unwrap());
    Courses::from_csv(horsage, fermes)
}

fn open_all_courses(courses_to_take: &mut [Course]) {
    for course in courses_to_take.iter_mut() {
        for g in course.lab_groups.iter_mut() {
            g.open = true;
        }
        for g in course.theo_groups.iter_mut() {
            g.open = true;
        }
    }
}

fn default_evaluation() -> EvaluationOption {
    EvaluationOption {
        day_off: 2,
        morning: 2,
        finish_early: 2,
    }
}

#[test]
fn assert_no_combination_possible() {
    let hellish_session = [
        "INF1900", "INF1015", "INF1600", "INF3405", "SSH3201", "SSH3501", "INF1007", "INF1015",
        "INF3500",
    ];
    let courses_to_take = get_h24_courses().get_courses(&hellish_session);
    let evaluation = default_evaluation();
    let options = SchedulesOptions {
        courses_to_take,
        max_nb_conflicts: 0,
        evaluation,
        max_size: 69,
    };
    let result = options.get_schedules().into_sorted_vec();
    assert_eq!(result.len(), 0);
}

#[test]
fn assert_possible() {
    let hellish_session = ["INF1900", "INF1015", "INF1600", "LOG1810"];
    let mut courses_to_take = get_h24_courses().get_courses(&hellish_session);
    let evaluation = default_evaluation();
    open_all_courses(&mut courses_to_take);
    let options = SchedulesOptions {
        courses_to_take,
        max_nb_conflicts: 1,
        evaluation,
        max_size: 69,
    };
    let result = options.get_schedules().into_sorted_vec();
    assert_eq!(result.len(), 69);
}
