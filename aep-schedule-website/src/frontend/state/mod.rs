use std::sync::atomic::{AtomicUsize, Ordering};

use aep_schedule_generator::{
    algorithm::{generation::SchedulesOptions, schedule::Schedule, scores::EvaluationOption},
    data::time::week::Week,
};
use leptos::prelude::*;
use reactive_course::ReactiveCourse;

pub mod action_add_course;
pub mod reactive_course;

#[derive(Copy, Clone)]
pub struct OptionState {
    pub first_generation_done: StoredValue<bool>,
    pub courses: RwSignal<Vec<ReactiveCourse>>,
    pub week: [RwSignal<u64>; 5],
    pub max_nb_conflicts: RwSignal<u8>,
    pub day_off: RwSignal<u8>,
    pub morning: RwSignal<i8>,
    pub finish_early: RwSignal<u8>,
    pub section_error: RwSignal<String>,
    pub personal_error: RwSignal<String>,
    pub step: RwSignal<u8>,
    pub hide: RwSignal<bool>,
    pub schedule: RwSignal<Vec<Schedule>>,
    pub max_size: StoredValue<AtomicUsize>,
}

impl OptionState {
    pub fn from_context() -> Self {
        use_context().unwrap()
    }

    pub fn submit(&self) {
        self.validate();
        if !self.first_generation_done.get_value() || self.step.get_untracked() < 5 {
            return;
        }
        self.generate();
    }

    pub fn submit_mobile(&self) {
        self.validate();
        if self.step.get() < 5 {
            self.hide.set(true);
            return;
        }
        self.generate();
    }

    fn validate(&self) {
        let mut options: SchedulesOptions = self.into();
        if options.courses_to_take.is_empty() {
            self.step.set(1);
            self.schedule.set(vec![]);
            return;
        }
        let mut impossible_courses = options.get_impossible_course().into_iter();
        if let Some(first_impossible_course) = impossible_courses.next() {
            let mut error = format!("Les sections des/du cours {}", first_impossible_course);
            for impossible_course in impossible_courses {
                error.push_str(", ");
                error.push_str(&impossible_course);
            }
            error.push_str(" sont toutes fermées.");
            self.section_error.set(error);
            self.step.set(2);
            self.schedule.set(vec![]);
            return;
        }
        self.section_error.set("".to_string());
        options.apply_personal_schedule();
        let mut impossible_courses = options.get_impossible_course().into_iter();
        if let Some(first_impossible_course) = impossible_courses.next() {
            let mut error = format!("Les sections des/du cours {}", first_impossible_course);
            for impossible_course in impossible_courses {
                error.push_str(", ");
                error.push_str(&impossible_course);
            }
            error.push_str(" sont en conflits avec les heures libres sélectionnées.");
            self.personal_error.set(error);
            self.step.set(3);
            self.schedule.set(vec![]);
            return;
        }
        self.personal_error.set("".to_string());
        self.step.update(|v| {
            if *v != 6 {
                *v = 5
            }
        });
    }

    fn generate(&self) {
        self.max_size
            .update_value(|v| v.store(8, Ordering::Relaxed));
        self.hide.set(true);
        self.gen();
        self.step.set(6);
    }

    pub fn regenerate(&self) {
        self.max_size.update_value(|size| {
            let _ = size.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| {
                Some(std::cmp::min(v * 2, 2usize.pow(10)))
            });
        });
        self.gen();
    }

    fn gen(&self) {
        let mut schedule_option: SchedulesOptions = self.into();
        schedule_option.apply_personal_schedule();
        let schedules = schedule_option.get_schedules().into_sorted_vec();
        self.schedule.set(schedules);
    }
}

impl Default for OptionState {
    fn default() -> Self {
        let courses: RwSignal<Vec<ReactiveCourse>> = RwSignal::new(vec![]);

        Self {
            first_generation_done: StoredValue::new(false),
            courses,
            max_nb_conflicts: RwSignal::new(0),
            week: std::array::from_fn(|_i| RwSignal::new(0)),
            day_off: RwSignal::new(3),
            morning: RwSignal::new(1),
            finish_early: RwSignal::new(1),
            section_error: RwSignal::new("".to_string()),
            personal_error: RwSignal::new("".to_string()),
            step: RwSignal::new(0),
            schedule: RwSignal::new(vec![]),
            hide: RwSignal::new(false),
            max_size: StoredValue::new(AtomicUsize::from(8)),
        }
    }
}

impl From<&OptionState> for SchedulesOptions {
    fn from(state: &OptionState) -> Self {
        let courses_to_take = state
            .courses
            .get_untracked()
            .into_iter()
            .map(|c| c.into())
            .collect();
        let mut max_size = 8;
        state
            .max_size
            .update_value(|v| max_size = v.load(Ordering::Relaxed));
        let max_nb_conflicts = state.max_nb_conflicts.get_untracked();
        let evaluation = EvaluationOption {
            day_off: state.day_off.get_untracked(),
            morning: state.morning.get_untracked(),
            finish_early: state.finish_early.get_untracked(),
        };
        let user_conflicts = Week::new(state.week.map(|s| s.get_untracked() << 2));
        Self {
            courses_to_take,
            max_nb_conflicts,
            evaluation,
            user_conflicts,
            max_size,
        }
    }
}
