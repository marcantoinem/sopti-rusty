use crate::frontend::components::{options::OptionsForms, schedules::SchedulesComponent};
use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (schedule, set_schedule) = create_signal(vec![]);

    view! {
        <div class="main-panel">
            <div class="left-panel">
                <OptionsForms write_signal=set_schedule/>
            </div>
            <div class="right-panel">
                <SchedulesComponent read_signal=schedule/>
            </div>
        </div>
    }
}
