use aep_schedule_generator::{
    algorithm::generation::SchedulesOptions,
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
        Self { courses_to_take }
    }
}
