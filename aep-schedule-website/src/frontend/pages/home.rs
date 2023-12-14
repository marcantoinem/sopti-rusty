use crate::frontend::components::{options::form::OptionsForms, schedules::SchedulesComponent};
use aep_schedule_generator::algorithm::generation::SchedulesOptions;
use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let action = create_action(|s: &SchedulesOptions| {
        let s = s.clone();
        logging::log!("{:?}", s);
        async move { s.get_schedules().into_sorted_vec() }
    });
    view! {
        <div class="main-panel">
            <div class="left-panel">
                <OptionsForms action=action/>
            </div>
            <div class="right-panel">
                <SchedulesComponent read_signal=action.value()/>
            </div>
        </div>
    }
}
