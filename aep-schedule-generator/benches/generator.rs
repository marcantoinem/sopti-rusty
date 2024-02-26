use divan::black_box;
use aep_schedule_generator::data::courses::Courses;
use std::io::BufReader;
use std::fs::File;
use aep_schedule_generator::algorithm::scores::EvaluationOption;
use aep_schedule_generator::data::course::Course;
use divan::Bencher;
use aep_schedule_generator::algorithm::generation::SchedulesOptions;

fn main() {
    divan::main();
}

fn get_h24_courses() -> Courses {
    let horsage = BufReader::new(File::open("horsage.csv").unwrap());
    let fermes = BufReader::new(File::open("fermes.csv").unwrap());
    Courses::from_csv(horsage, fermes)
}

fn default_evaluation() -> EvaluationOption {
    EvaluationOption {
        day_off: 2,
        morning: 2,
        finish_early: 2,
    }
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

#[divan::bench]
fn hellish_schedule(bencher: Bencher) {
    let hellish_session = [
        "INF1900", "INF1015", "INF1600", "INF3405", "SSH3201", "SSH3501", "INF3500",
    ];
    let mut courses_to_take = get_h24_courses().get_courses(&hellish_session);
    open_all_courses(&mut courses_to_take);
    let evaluation = default_evaluation();
    let options = SchedulesOptions {
        courses_to_take,
        max_nb_conflicts: 4,
        evaluation,
        max_size: 69,
    };
    
    bencher.bench_local(|| {
        black_box(&options).get_schedules().into_sorted_vec();
    });
}