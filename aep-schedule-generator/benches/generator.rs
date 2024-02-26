use aep_schedule_generator::algorithm::generation::SchedulesOptions;
use aep_schedule_generator::algorithm::schedule::Schedule;
use aep_schedule_generator::algorithm::scores::{EvaluationOption, Score};
use aep_schedule_generator::algorithm::taken_courses::TakenCourses;
use aep_schedule_generator::data::course::Course;
use aep_schedule_generator::data::courses::Courses;
use aep_schedule_generator::data::time::weeks::Weeks;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::BufReader;

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

fn clone_schedule(c: &mut Criterion) {
    let hellish_session = [
        "INF1900", "INF1015", "INF1600", "INF3405", "SSH3201", "SSH3501",
    ];
    let courses_to_take = get_h24_courses().get_courses(&hellish_session);
    let schedule = Schedule {
        score: Score::default(),
        week: Weeks::default(),
        conflicts: 0,
        taken_courses: TakenCourses::default(),
        courses: &courses_to_take,
    };

    c.bench_function("Clone schedule benchmark", |b| {
        b.iter(|| black_box(&schedule).clone());
    });
}

fn evaluate_schedule(c: &mut Criterion) {
    let hellish_session = [
        "INF1900", "INF1015", "INF1600", "INF3405", "SSH3201", "SSH3501",
    ];
    let courses_to_take = get_h24_courses().get_courses(&hellish_session);
    let mut schedule = Schedule {
        score: Score::default(),
        week: Weeks::default(),
        conflicts: 0,
        taken_courses: TakenCourses::default(),
        courses: &courses_to_take,
    };

    c.bench_function("Evaluate schedule benchmark", |b| {
        b.iter(|| black_box(&mut schedule.score).evaluate(default_evaluation()));
    });
}

fn hellish_schedule(c: &mut Criterion) {
    let hellish_session = [
        "INF1900", "INF1015", "INF1600", "INF3405", "SSH3201", "SSH3501", "INF3405",
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

    c.bench_function("Hellish schedule benchmark", |b| {
        b.iter(|| black_box(&options).get_schedules().into_sorted_vec());
    });
}

fn bench_generator(c: &mut Criterion) {
    evaluate_schedule(c);
    clone_schedule(c);
    hellish_schedule(c);
}

criterion_group!(benches, bench_generator);
criterion_main!(benches);
