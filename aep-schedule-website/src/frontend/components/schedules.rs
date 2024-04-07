use crate::{backend::routes::get_calendar, frontend::components::schedule::ScheduleComponent};
use aep_schedule_generator::algorithm::schedule::Schedule;
use leptos::*;

#[component]
pub fn SchedulesComponent(read_signal: RwSignal<Option<Vec<Schedule>>>) -> impl IntoView {
    view! {
        {move || read_signal.get().unwrap_or_default().into_iter().rev().map(|schedule| {
                view! {
                    <Await
                        future=get_calendar
                        children=move |calendar| {
                            let calendar = calendar.clone().unwrap();
                            let schedule = schedule.clone();
                            view! {
                                <ScheduleComponent schedule calendar/>
                            }
                        }
                    />
                }
            }).collect_view()
        }
    }
}
