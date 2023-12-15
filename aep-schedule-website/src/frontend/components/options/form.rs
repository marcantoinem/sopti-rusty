use crate::frontend::components::options::{
    courses_selector::CoursesSelector, optimizations::SelectOptimizations, state::OptionState,
};
use aep_schedule_generator::algorithm::{generation::SchedulesOptions, schedule::Schedule};
use leptos::*;
use thaw::InputNumber;

#[component]
pub fn OptionsForms(action: Action<SchedulesOptions, Vec<Schedule>>) -> impl IntoView {
    let state = OptionState::default();

    let on_submit = move |_| action.dispatch((&state).into());

    view! {
        <h1>"Options de générations"</h1>
        <CoursesSelector state=state/>
        <label for="conflicts">"Nombre de conflits maximum"</label>
        <div class="input-item"><InputNumber value=state.max_nb_conflicts step=1/></div>
        <br/>
        <SelectOptimizations state=state/>
        <button on:click=on_submit class="submit">"Générer les horaires"</button>
    }
}
