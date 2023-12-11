use aep_schedule_generator::{algorithm::schedule::Schedule, data::courses::SchedulesOptions};
use leptos::*;
use leptos_router::*;

use crate::backend::routes::get_schedules;

pub fn create_options() -> SchedulesOptions {
    let courses = use_query_map()().get("cours").cloned().unwrap_or_default();
    let courses_to_take = courses.split(",").map(|s| s.trim().to_string()).collect();
    SchedulesOptions { courses_to_take }
}

#[component]
pub fn OptionsForms(write_signal: WriteSignal<Vec<(usize, Schedule)>>) -> impl IntoView {
    let query = use_query_map();
    let cours = move || query().get("cours").cloned().unwrap_or_default();
    let submit_schedules = move |_| {
        spawn_local(async move {
            let options = create_options();
            let schedules = get_schedules(options).await.unwrap_or_default();
            let schedules = schedules.into_iter().enumerate().collect();
            write_signal.set(schedules);
        });
    };

    view! {
        <h2>"Schedule Generation"</h2>
        <Form method="GET" action="" on:submit=submit_schedules>
            // input names determine query string key
            <input type="text" name="cours" value=cours/>
            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit"/>
        </Form>
    }
}
