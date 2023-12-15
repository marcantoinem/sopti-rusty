use aep_schedule_generator::{
    algorithm::{generation::SchedulesOptions, scores::EvaluationOption},
    data::{course::Course, groups::Groups},
};
use compact_str::CompactString;
use leptos::*;

#[derive(Copy, Clone)]
pub struct OptionState {
    pub selections: (
        ReadSignal<Vec<ReactiveCourse>>,
        WriteSignal<Vec<ReactiveCourse>>,
    ),
    pub max_nb_conflicts: (ReadSignal<u8>, WriteSignal<u8>),
    pub day_off: (ReadSignal<u8>, WriteSignal<u8>),
    pub morning: (ReadSignal<i8>, WriteSignal<i8>),
    pub finish_early: (ReadSignal<u8>, WriteSignal<u8>),
}

#[derive(Clone)]
pub struct ReactiveCourse {
    pub sigle: CompactString,
    pub name: String,
    pub theo_groups: (ReadSignal<Groups>, WriteSignal<Groups>),
    pub lab_groups: (ReadSignal<Groups>, WriteSignal<Groups>),
    pub nb_credit: usize,
}

impl Default for OptionState {
    fn default() -> Self {
        Self {
            selections: create_signal(vec![]),
            max_nb_conflicts: create_signal(0),
            day_off: create_signal(0),
            morning: create_signal(0),
            finish_early: create_signal(0),
        }
    }
}

impl From<ReactiveCourse> for Course {
    fn from(value: ReactiveCourse) -> Self {
        Self {
            sigle: value.sigle,
            name: value.name,
            theo_groups: value.theo_groups.0.get(),
            lab_groups: value.lab_groups.0.get(),
            nb_credit: value.nb_credit,
        }
    }
}

impl From<Course> for ReactiveCourse {
    fn from(value: Course) -> Self {
        Self {
            sigle: value.sigle,
            name: value.name,
            theo_groups: create_signal(value.theo_groups),
            lab_groups: create_signal(value.lab_groups),
            nb_credit: value.nb_credit,
        }
    }
}

impl From<&OptionState> for SchedulesOptions {
    fn from(state: &OptionState) -> Self {
        let courses_to_take = state
            .selections
            .0
            .get()
            .into_iter()
            .map(|c| c.into())
            .collect();
        let max_nb_conflicts = state.max_nb_conflicts.0.get();
        let evaluation = EvaluationOption {
            day_off: state.day_off.0.get(),
            morning: state.morning.0.get(),
            finish_early: state.finish_early.0.get(),
        };
        Self {
            courses_to_take,
            max_nb_conflicts,
            evaluation,
        }
    }
}
