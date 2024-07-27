use crate::frontend::{
    components::{
        common::number_input::NumberInput,
        options::{
            courses_selector::CoursesSelector, optimizations::SelectOptimizations,
            state::OptionState,
        },
    },
    pages::generator::FirstGenerationDone,
};
use aep_schedule_generator::algorithm::{generation::SchedulesOptions, schedule::Schedule};
use leptos::*;

#[component]
pub fn OptionsForms(action: Action<SchedulesOptions, Vec<Schedule>>) -> impl IntoView {
    let state: OptionState = use_context().unwrap();

    let first_generation_done: FirstGenerationDone = use_context().unwrap();
    let submit = move || {
        if !first_generation_done.0.get() {
            return;
        }
        action.dispatch((&state).into());
    };

    let submit_mobile = move |_| action.dispatch((&state).into());

    view! {
        <CoursesSelector state=state submit/>
        <span class="grow"></span>
        <div class="row-container input-item auto-bottom"><p>"Nombre de conflits maximum"</p><NumberInput value=state.max_nb_conflicts max=127/></div>
        <SelectOptimizations state=state submit/>
        <button on:click=submit_mobile class="lg:hidden select-none rounded-lg bg-amber-500 py-2 text-xl px-4 w-64 self-center text-center align-middle text-black shadow-md shadow-amber-500/20 transition-all hover:shadow-lg hover:shadow-amber-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none">"Générer les horaires"</button>
    }
}
