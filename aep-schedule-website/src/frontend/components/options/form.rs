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
    set_hide: WriteSignal<bool>,
) -> impl IntoView {
    let state = OptionState::default();

    let on_submit = move |_| {
        set_hide(true);
        action.dispatch((&state).into())
    };

    view! {
        <h1 class="title">"Options de générations"</h1>
        <CoursesSelector state=state/>
        <span class="spacer"></span>
        <div class="row-container input-item auto-bottom"><p>"Nombre de conflits maximum"</p><NumberInput value=state.max_nb_conflicts max=127/></div>
        <SelectOptimizations state=state/>
        <button on:click=on_submit class="submit">"Générer les horaires"</button>
    }
}
