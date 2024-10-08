use std::sync::atomic::{AtomicUsize, Ordering};

use aep_schedule_generator::{
    algorithm::{generation::SchedulesOptions, schedule::Schedule, scores::EvaluationOption},
    data::time::week::Week,
};
use leptos::*;
use reactive_course::ReactiveCourse;

use crate::backend::routes::get_course;

pub mod reactive_course;

#[derive(Copy, Clone)]
pub struct OptionState {
    pub stored_courses: StoredValue<Vec<ReactiveCourse>>,
    pub action_courses: Action<String, Vec<ReactiveCourse>>,
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

    pub fn validate(self) {
        let mut options: SchedulesOptions = (&self).into();
        if options.courses_to_take.is_empty() {
            self.step.set(1);
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
            return;
        }
        self.personal_error.set("".to_string());
        self.step.update(|v| {
            if *v != 6 {
                *v = 5
            }
        });
    }

    pub fn generate(&self) {
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
        let stored_courses: StoredValue<Vec<ReactiveCourse>> = store_value(vec![]);

        let action_courses = create_action(move |sigle: &String| {
            let sigle = sigle.clone();
            async move {
                if let Ok(c) = get_course(sigle).await {
                    if !stored_courses
                        .get_value()
                        .iter()
                        .any(|react_c| react_c.sigle == c.sigle)
                    {
                        stored_courses.update_value(|courses| courses.push(c.into()));
                    }
                }
                stored_courses.get_value()
            }
        });

        Self {
            stored_courses,
            action_courses,
            max_nb_conflicts: create_rw_signal(0),
            week: std::array::from_fn(|_i| create_rw_signal(0)),
            day_off: create_rw_signal(3),
            morning: create_rw_signal(1),
            finish_early: create_rw_signal(1),
            section_error: create_rw_signal("".to_string()),
            personal_error: create_rw_signal("".to_string()),
            step: create_rw_signal(0),
            schedule: create_rw_signal(vec![]),
            hide: create_rw_signal(false),
            max_size: store_value(AtomicUsize::from(8)),
        }
    }
}

impl From<&OptionState> for SchedulesOptions {
    fn from(state: &OptionState) -> Self {
        let courses_to_take = state
            .action_courses
            .value()
            .get()
            .unwrap_or_default()
            .into_iter()
            .map(|c| c.into())
            .collect();
        let mut max_size = 8;
        state
            .max_size
            .update_value(|v| max_size = v.load(Ordering::Relaxed));
        let max_nb_conflicts = state.max_nb_conflicts.get();
        let evaluation = EvaluationOption {
            day_off: state.day_off.get(),
            morning: state.morning.get(),
            finish_early: state.finish_early.get(),
        };
        let user_conflicts = Week::new(state.week.map(|s| s.get() << 2));
        Self {
            courses_to_take,
            max_nb_conflicts,
            evaluation,
            user_conflicts,
            max_size,
        }
    }
}
