use crate::frontend::components::schedule::ScheduleComponent;
use aep_schedule_generator::algorithm::schedule::Schedule;
use leptos::*;

#[component]
pub fn SchedulesComponent(read_signal: RwSignal<Option<Vec<Schedule>>>) -> impl IntoView {
    view! {
        {move || read_signal.get().unwrap_or_default().into_iter().rev().map(|s| {
                view! {
                    <ScheduleComponent schedule=s/>
                }
            }).collect_view()
        }
    }
}
