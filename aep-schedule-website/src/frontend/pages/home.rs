use crate::{
    backend::routes::get_schedules,
    frontend::components::{options::form::OptionsForms, schedules::SchedulesComponent},
};
use aep_schedule_generator::data::courses::SchedulesOptions;
use leptos::*;

/// Renders the home page of your application.
#[component]
pub fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let action = create_action(|s: &SchedulesOptions| {
        let s = s.clone();
        async move { get_schedules(s).await.unwrap() }
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
