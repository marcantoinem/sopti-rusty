use crate::frontend::{
    components::common::autocomplete::{AutoComplete, AutoCompleteOption},
    state::action_add_course::ActionAddCourse,
};
use aep_schedule_generator::data::course::CourseName;
use leptos::prelude::*;

#[component]
pub fn SearchCourse(
    all_courses: Result<Vec<CourseName>, ServerFnError>,
    set_active_tab: WriteSignal<String>,
) -> impl IntoView {
    let Ok(courses) = all_courses else {
        return None;
    };
    let courses = courses
        .into_iter()
        .map(|c| AutoCompleteOption::new(c.sigle.clone(), c.sigle + " - " + &c.name))
        .collect();

    let action_courses = ActionAddCourse::from_context().0;
    let on_submit = move |sigle: String| {
        set_active_tab(sigle.clone());
        action_courses.dispatch(sigle);
    };

    Some(view! {
        <AutoComplete
            suggestion_list=courses
            placeholder="Cours"
            submit=on_submit
            id="course-submitter"
        />
    })
}
