use std::sync::Arc;

use crate::components::icons::warning_circle::WarningCircle;
use crate::components::options::todo::Todo;
use crate::state::OptionState;
use crate::{components::schedule::ScheduleComponent, routes::get_calendar};
use leptos::prelude::*;

#[component]
pub fn SchedulesComponent() -> impl IntoView {
    let state = OptionState::from_context();

    view! {
        <Await
            future=get_calendar()
            children=move |calendar| {
                let calendar = Arc::new(calendar.clone().unwrap());
                view! {
                    {move || {
                        let bad_generation = state.schedule.get().is_empty();
                        let generated = state.step.get() == 6;
                        if !generated || bad_generation {
                            Some(view! { <Todo /> })
                        } else {
                            None
                        }
                    }}
                    {move || {
                        let bad_generation = state.schedule.get().is_empty();
                        let generated = state.step.get() == 6;
                        if generated && bad_generation {
                            Some(
                                view! {
                                    <div class="p-4 gap-4 max-w-3xl flex flex-row items-center text-justify bg-red-500 text-white">
                                        <WarningCircle size="4em" />
                                        <span>
                                            "Aucun horaire n'a pu être généré, augmentez le nombre de conflits ou ouvrez des sections. Probablement que deux groupes sont toujours en conflits."
                                        </span>
                                    </div>
                                },
                            )
                        } else {
                            None
                        }
                    }}
                    <For
                        each=move || state.schedule.get()
                        key=|course| course.id
                        children=move |schedule| {
                            let calendar = Arc::clone(&calendar);
                            view! { <ScheduleComponent schedule calendar /> }
                        }
                    />
                }
            }
        />
    }
}
