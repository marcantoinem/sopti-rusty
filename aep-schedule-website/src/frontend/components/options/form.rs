use aep_schedule_generator::{algorithm::schedule::Schedule, data::courses::SchedulesOptions};
use leptos::html::Input;
use leptos::*;
use leptos_router::*;

pub fn create_options(inputref: NodeRef<Input>) -> SchedulesOptions {
    let courses = inputref.get().unwrap().value();
    let courses_to_take = courses.split(",").map(|s| s.trim().to_string()).collect();
    SchedulesOptions { courses_to_take }
}

#[component]
pub fn OptionsForms(action: Action<SchedulesOptions, Vec<Schedule>>) -> impl IntoView {
    let query = use_query_map();
    let cours = move || query().get("cours").cloned().unwrap_or_default();
    let cours_ref = create_node_ref::<Input>();
    let on_submit = move |_| action.dispatch(create_options(cours_ref));

    view! {
        <h2>"Options de générations"</h2>
        <Form method="GET" action="" on:formdata=on_submit>
            // input names determine query string key
            <input type="text" name="cours" value=cours node_ref=cours_ref/>
            // submitting should cause a client-side
            // navigation, not a full reload
            <br/>
            <input type="submit" value="Générer les horaires"/>
        </Form>
    }
}
