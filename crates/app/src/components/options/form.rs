use crate::{
    components::{
        common::number_input::NumberInput,
        options::{courses_selector::CoursesSelector, optimizations::SelectOptimizations},
    },
    state::OptionState,
};
use leptos::prelude::*;

#[component]
pub fn OptionsForms() -> impl IntoView {
    let state = OptionState::from_context();

    let submit = move || {
        state.submit();
    };

    let submit_mobile = move |_| {
        state.submit_mobile();
    };

    view! {
        <CoursesSelector submit />
        <span class="grow"></span>
        <NumberInput
            value=state.max_nb_conflicts
            max=127
            label="Nombre de période de cours en conflits maximum: "
            submit
        />
        <SelectOptimizations submit />
        <button
            on:pointerdown=submit_mobile
            class="lg:hidden select-none rounded-lg bg-amber-500 py-2 text-xl px-4 w-64 self-center text-center align-middle text-black shadow-md shadow-amber-500/20 transition-all hover:shadow-lg hover:shadow-amber-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none"
        >
            "Générer les horaires"
        </button>
    }
}
