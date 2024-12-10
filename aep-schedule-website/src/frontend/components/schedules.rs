use std::sync::Arc;

use crate::frontend::components::icons::warning_circle::WarningCircle;
use crate::frontend::components::options::todo::Todo;
use crate::frontend::state::OptionState;
use crate::{backend::routes::get_calendar, frontend::components::schedule::ScheduleComponent};
use leptos::prelude::*;

#[component]
pub fn SchedulesComponent() -> impl IntoView {
    let state = OptionState::from_context();

    view! {
        <Await
            future=get_calendar()
            children=move |calendar| {
                let calendar = Arc::new(calendar.clone().unwrap());
                let bad_generation = state.schedule.get().is_empty();
                let generated = state.step.get() == 6;
                match generated && !bad_generation {
                    true => {
                        view! {
                            <For
                                each=move || state.schedule.get()
                                key=|course| course.id
                                children=move |schedule| {
                                    let calendar = Arc::clone(&calendar);
                                    view! { <ScheduleComponent schedule calendar /> }
                                }
                            />
                        }
                            .into_any()
                    }
                    _ => {
                        view! {
                            <Todo />
                            <For
                                each=move || state.schedule.get()
                                key=|course| course.id
                                children=move |schedule| {
                                    let calendar = Arc::clone(&calendar);
                                    view! { <ScheduleComponent schedule calendar /> }
                                }
                            />
                            {match generated && bad_generation {
                                true => {
                                    Some(
                                        view! {
                                            <div class="warning-box">
                                                <WarningCircle size="4em" />
                                                <span>
                                                    "Aucun horaire n'a pu être généré, augmentez le nombre de conflits ou ouvrez des sections. Probablement que deux groupes sont toujours en conflits."
                                                </span>
                                            </div>
                                        },
                                    )
                                }
                                false => None,
                            }}
                        }
                            .into_any()
                    }
                }
            }
        />
    }
}
