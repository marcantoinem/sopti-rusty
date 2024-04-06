use crate::frontend::components::{options::form::OptionsForms, schedules::SchedulesComponent};
use aep_schedule_generator::algorithm::generation::SchedulesOptions;
use leptos::*;
use phosphor_leptos::{CaretDoubleRight, IconWeight};

#[component]
pub fn GeneratorPage() -> impl IntoView {
    // Creates a reactive value to update the button
    let action = create_action(|s: &SchedulesOptions| {
        let s = s.clone();
        async move { s.get_schedules().into_sorted_vec() }
    });
    let (hide, set_hide) = create_signal(false);
    view! {
        <aside class="left-panel" class=("hide-left-panel", hide)>
            <OptionsForms action=action set_hide/>
        </aside>
        <section class="right-panel">
            <SchedulesComponent read_signal=action.value()/>
        </section>
        <button on:click=move |_| {set_hide(false)} id="go-back"><CaretDoubleRight weight=IconWeight::Regular size="3vh"/></button>
    }
}
