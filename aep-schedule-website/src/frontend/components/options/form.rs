use crate::frontend::components::options::{courses_selector::CoursesSelector, state::OptionState};
use aep_schedule_generator::algorithm::{generation::SchedulesOptions, schedule::Schedule};
use leptos::*;

pub fn create_options() -> SchedulesOptions {
    // let courses = inputref.get().unwrap().value();
    let courses_to_take = vec![];
    SchedulesOptions { courses_to_take }
}

#[component]
pub fn OptionsForms(action: Action<SchedulesOptions, Vec<Schedule>>) -> impl IntoView {
    let state = OptionState::default();

    let on_submit = move |_| action.dispatch((&state).into());

    view! {
        <h1>"Options de générations"</h1>
        // input names determine query string key
        <CoursesSelector state=state/>
        // submitting should cause a client-side
        // navigation, not a full reload
        <br/>
        <button on:click=on_submit class="submit">"Générer les horaires"</button>
    }
}
