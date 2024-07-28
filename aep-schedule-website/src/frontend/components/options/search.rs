use super::state::ReactiveCourse;
use crate::frontend::components::common::autocomplete::{AutoComplete, AutoCompleteOption};
use aep_schedule_generator::data::course::CourseName;
use leptos::*;

#[component]
pub fn SearchCourse(
    courses: Result<Vec<CourseName>, ServerFnError>,
    set_active_tab: WriteSignal<String>,
    action_courses: Action<String, Vec<ReactiveCourse>>,
) -> impl IntoView {
    let Ok(courses) = courses else {
        return None;
    };
    let courses = courses
        .into_iter()
        .map(|c| AutoCompleteOption::new(c.sigle.clone(), c.sigle + " - " + &c.name))
        .collect();

    let on_submit = move |sigle: String| {
        set_active_tab(sigle.clone());
        action_courses.dispatch(sigle);
    };

    Some(view! {
        <AutoComplete suggestion_list=courses placeholder="Cours" submit=on_submit id="course-submitter"/>
    })
}
