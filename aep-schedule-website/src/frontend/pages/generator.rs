use crate::frontend::components::{options::form::OptionsForms, schedules::SchedulesComponent};
use aep_schedule_generator::algorithm::generation::SchedulesOptions;
use leptos::*;

#[component]
pub fn GeneratorPage() -> impl IntoView {
    // Creates a reactive value to update the button
    let action = create_action(|s: &SchedulesOptions| {
        let s = s.clone();
        async move { s.get_schedules().into_sorted_vec() }
    });
    view! {
        <aside class="left-panel">
            <OptionsForms action=action/>
        </aside>
        <section class="right-panel">
            <SchedulesComponent read_signal=action.value()/>
        </section>
    }
}
