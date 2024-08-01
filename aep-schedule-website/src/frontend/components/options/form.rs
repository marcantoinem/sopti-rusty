use crate::frontend::{
    components::{
        common::number_input::NumberInput,
        options::{courses_selector::CoursesSelector, optimizations::SelectOptimizations},
    },
    pages::generator::FirstGenerationDone,
    state::OptionState,
};
use leptos::*;

#[component]
pub fn OptionsForms() -> impl IntoView {
    let state = OptionState::from_context();

    let first_generation_done: FirstGenerationDone = use_context().unwrap();
    let submit = move || {
        state.validate();
        if !first_generation_done.0.get() || state.step.get() < 5 {
            return;
        }
        state.generate();
    };

    create_local_resource(state.action_courses.pending(), move |_| {
        submit();
        async move {}
    });

    let submit_mobile = move |_| {
        state.validate();
        if state.step.get() < 5 {
            state.hide.set(true);
            return;
        }
        state.generate();
    };

    view! {
        <CoursesSelector state=state submit/>
        <span class="grow"></span>
        <NumberInput value=state.max_nb_conflicts max=127 label="Nombre de période de cours en conflits maximum: " submit/>
        <SelectOptimizations state=state submit/>
        <button on:click=submit_mobile class="lg:hidden select-none rounded-lg bg-amber-500 py-2 text-xl px-4 w-64 self-center text-center align-middle text-black shadow-md shadow-amber-500/20 transition-all hover:shadow-lg hover:shadow-amber-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none">"Générer les horaires"</button>
    }
}
