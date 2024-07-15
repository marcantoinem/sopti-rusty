use crate::frontend::components::{
    common::number_input::NumberInput,
    options::{
        courses_selector::CoursesSelector, optimizations::SelectOptimizations, state::OptionState,
    },
};
use aep_schedule_generator::algorithm::{generation::SchedulesOptions, schedule::Schedule};
use leptos::*;

#[component]
pub fn OptionsForms(
    action: Action<SchedulesOptions, Vec<Schedule>>,
) -> impl IntoView {
    let state = OptionState::default();

    let submit = move || {
        action.dispatch((&state).into())
    };

    view! {
        <h1 class="title">"Options de générations"</h1>
        <CoursesSelector state=state submit/>
        <span class="spacer"></span>
        <div class="row-container input-item auto-bottom"><p>"Nombre de conflits maximum"</p><NumberInput value=state.max_nb_conflicts max=127/></div>
        <SelectOptimizations state=state submit/>
        <button on:click=move |_| {submit()} class="submit">"Générer les horaires"</button>
    }
}
