use std::rc::Rc;

use crate::frontend::components::options::todo::Todo;
use crate::frontend::pages::generator::FirstGenerationDone;
use crate::frontend::state::OptionState;
use crate::{backend::routes::get_calendar, frontend::components::schedule::ScheduleComponent};
use leptos::*;

#[component]
pub fn SchedulesComponent() -> impl IntoView {
    let state = OptionState::from_context();
    let first_generation_done: FirstGenerationDone = use_context().unwrap();

    view! {
        <Await
            future=get_calendar
            children=move |calendar| {
                match state.step.get() == 5 && first_generation_done.0.get() {
                    true => {
                        let calendar = Rc::new(calendar.clone().unwrap());
                        view !{
                            <For
                                each=move || {state.schedule.get()}
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
                        <Todo/>
                    }
                }
            }
        />
    }
}
