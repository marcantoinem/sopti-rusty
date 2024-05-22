use crate::frontend::components::{options::form::OptionsForms, schedules::SchedulesComponent};
use aep_schedule_generator::algorithm::generation::SchedulesOptions;
use leptos::*;
use phosphor_leptos::{CaretDoubleRight, IconWeight};

#[component]
pub fn GeneratorPage() -> impl IntoView {
    let (hide, set_hide) = create_signal(false);
    // Creates a reactive value to update the button
    let action = create_action( move|s: &SchedulesOptions| {
        let s = s.clone();
        set_hide(true);
        async move { s.get_schedules().into_sorted_vec() }
    });
    view! {
        <aside class="left-panel" class=("hide-left-panel", hide)>
            <OptionsForms action=action/>
        </aside>
        <section class="right-panel">
            <SchedulesComponent read_signal=action.value()/>
        </section>
        <button on:click=move |_| {set_hide(false)} id="go-back"><CaretDoubleRight weight=IconWeight::Regular size="3vh"/></button>
    }
}
