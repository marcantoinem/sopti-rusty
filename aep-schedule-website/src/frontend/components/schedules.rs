use std::rc::Rc;

use crate::frontend::components::options::todo::Todo;
use crate::{backend::routes::get_calendar, frontend::components::schedule::ScheduleComponent};
use aep_schedule_generator::algorithm::generation::SchedulesOptions;
use aep_schedule_generator::algorithm::schedule::Schedule;
use leptos::*;

#[component]
pub fn SchedulesComponent(
    read_signal: RwSignal<Option<Vec<Schedule>>>,
    action: Action<SchedulesOptions, Vec<Schedule>>,
    section_error: RwSignal<String>,
    personal_error: RwSignal<String>,
    step: ReadSignal<u8>,
) -> impl IntoView {
    view! {
        <Await
            future=get_calendar
            children=move |calendar| {
                match (read_signal.get(), step.get() == 5) {
                    (Some(_), true) => {
                        let calendar = Rc::new(calendar.clone().unwrap());
                        view !{
                            <For
                                each=move || {read_signal.get().unwrap()}
                                key= |course| course.id
                                children= move |schedule| {
                                    let calendar = Rc::clone(&calendar);
                                    view !{
                                        <ScheduleComponent schedule calendar/>
                                    }
                                }
                            />
                        }
                   },
                    _ => view ! {
                        <Todo action step section_error personal_error/>
                    }
                }
            }
        />
    }
}
