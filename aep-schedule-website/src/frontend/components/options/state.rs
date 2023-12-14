use aep_schedule_generator::{algorithm::generation::SchedulesOptions, data::course::Course};
use leptos::*;

#[derive(Copy, Clone)]
pub struct OptionState {
    pub selections: (ReadSignal<Vec<Course>>, WriteSignal<Vec<Course>>),
}

impl Default for OptionState {
    fn default() -> Self {
        Self {
            selections: create_signal(vec![]),
        }
    }
}

impl From<&OptionState> for SchedulesOptions {
    fn from(state: &OptionState) -> Self {
        let courses_to_take = state.selections.0.get();
        Self { courses_to_take }
    }
}
