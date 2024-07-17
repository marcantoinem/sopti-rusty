use crate::frontend::components::options::todo::Todo;
use crate::{backend::routes::get_calendar, frontend::components::schedule::ScheduleComponent};
use aep_schedule_generator::algorithm::schedule::Schedule;
use leptos::*;

#[component]
pub fn SchedulesComponent(read_signal: RwSignal<Option<Vec<Schedule>>>) -> impl IntoView {
    view! {
        <Await
            future=get_calendar
            children=move |calendar| {
                match read_signal.get() {
                    Some(result) => result.into_iter().rev().map(|schedule| {
                    let calendar = calendar.clone().unwrap();
                    let schedule = schedule.clone();
                    view! {
                        <ScheduleComponent schedule calendar/>
                    }
                }).collect_view(),
                    None => view ! {
                        <Todo/>
                    }
                }
            }
        />
    }
}
