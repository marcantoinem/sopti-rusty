use crate::backend::routes::get_schedules;
use leptos::ev::SubmitEvent;
use leptos::html::*;
use leptos::*;
use leptos_router::*;

#[derive(Default)]
struct SchedulesOptions {
    courses_to_take: Vec<String>,
}

impl SchedulesOptions {
    fn new(courses_to_take: String) -> Self {
        let courses_to_take = courses_to_take
            .split(",")
            .map(|s| s.trim().to_string())
            .collect();
        SchedulesOptions { courses_to_take }
    }
    // fn courses_to_take(self) -> String {
    //     self.courses_to_take[0]
    // }
}

#[component]
pub fn SchedulesOptions() -> impl IntoView {
    let query = use_query_map();

    let user = create_resource(|| (), |_| async { get_schedules().await });

    let name = move || query().get("name").cloned().unwrap_or_default();
    let number = move || query().get("number").cloned().unwrap_or_default();
    let select = move || query().get("select").cloned().unwrap_or_default();
    view! {
        <h2>"Manual Submission"</h2>
        <Form method="GET" action="" on:submit=move |_| { spawn_local(async { let _ = get_schedules().await; })}>
            // input names determine query string key
            <input type="text" name="name" value=name/>
            <input type="number" name="number" value=number/>
            <select name="select">
                // `selected` will set which starts as selected
                <option selected=move || select() == "A">
                    "A"
                </option>
                <option selected=move || select() == "B">
                    "B"
                </option>
                <option selected=move || select() == "C">
                    "C"
                </option>
            </select>
            // submitting should cause a client-side
            // navigation, not a full reload
            <input type="submit"/>
        </Form>
        <Suspense fallback=move || view! {<p>"Loading User"</p> }>
            <p>test</p>
        </Suspense>
    }
}
