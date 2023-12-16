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
    pub max_nb_conflicts: RwSignal<u8>,
    pub day_off: RwSignal<u8>,
    pub morning: RwSignal<i8>,
    pub finish_early: RwSignal<u8>,
}

#[derive(Clone)]
pub struct ReactiveCourse {
    pub sigle: CompactString,
    pub name: String,
    pub theo_groups: Groups,
    pub lab_groups: Groups,
    pub theo_open: Vec<RwSignal<bool>>,
    pub lab_open: Vec<RwSignal<bool>>,
    pub nb_credit: usize,
}

impl Default for OptionState {
    fn default() -> Self {
        Self {
            selections: create_signal(vec![]),
            max_nb_conflicts: create_rw_signal(0),
            day_off: create_rw_signal(3),
            morning: create_rw_signal(0),
            finish_early: create_rw_signal(1),
        }
    }
}

impl From<ReactiveCourse> for Course {
    fn from(mut value: ReactiveCourse) -> Self {
        for (i, open) in value.theo_open.iter().enumerate() {
            value.theo_groups[i].open = open.get();
        }
        for (i, open) in value.lab_open.iter().enumerate() {
            value.lab_groups[i].open = open.get();
        }
        Self {
            sigle: value.sigle,
            name: value.name,
            theo_groups: value.theo_groups,
            lab_groups: value.lab_groups,
            nb_credit: value.nb_credit,
        }
    }
}

impl From<Course> for ReactiveCourse {
    fn from(value: Course) -> Self {
        let theo_open = value
            .theo_groups
            .iter()
            .map(|g| create_rw_signal(g.open))
            .collect();
        let lab_open = value
            .lab_groups
            .iter()
            .map(|g| create_rw_signal(g.open))
            .collect();
        Self {
            sigle: value.sigle,
            name: value.name,
            theo_groups: value.theo_groups,
            lab_groups: value.lab_groups,
            theo_open,
            lab_open,
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
        let max_nb_conflicts = state.max_nb_conflicts.get();
        let evaluation = EvaluationOption {
            day_off: state.day_off.get(),
            morning: state.morning.get(),
            finish_early: state.finish_early.get(),
        };
        Self {
            courses_to_take,
            max_nb_conflicts,
            evaluation,
        }
    }
}
