use crate::frontend::components::options::{courses_selector::CoursesSelector, state::OptionState};
use aep_schedule_generator::algorithm::{generation::SchedulesOptions, schedule::Schedule};
use leptos::*;

#[component]
pub fn OptionsForms(action: Action<SchedulesOptions, Vec<Schedule>>) -> impl IntoView {
    let state = OptionState::default();

    let on_submit = move |_| action.dispatch((&state).into());
    let (conflicts, set_conflicts) = state.max_nb_conflicts;

    view! {
        <h1>"Options de générations"</h1>
        <CoursesSelector state=state/>
        <label for="conflicts">"Nombre de conflits maximum"</label>
        <input type="number" name="conflicts" prop:value=conflicts min="0" on:change=move |ev| {
            set_conflicts(event_target_value(&ev).parse::<u8>().unwrap())
        }/>
        <br/>
        <button on:click=on_submit class="submit">"Générer les horaires"</button>
    }
}
