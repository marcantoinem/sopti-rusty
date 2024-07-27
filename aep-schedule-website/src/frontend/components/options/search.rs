use super::state::ReactiveCourse;
use crate::{
    backend::routes::get_course,
    frontend::components::common::autocomplete::{AutoComplete, AutoCompleteOption},
};
use aep_schedule_generator::data::course::CourseName;
use leptos::*;

#[component]
pub fn SearchCourse<F>(
    courses: Result<Vec<CourseName>, ServerFnError>,
    set_selections: WriteSignal<Vec<ReactiveCourse>>,
    set_active_tab: WriteSignal<String>,
    submit: F,
) -> impl IntoView
where
    F: Fn() + Copy + 'static,
{
    let Ok(courses) = courses else {
        return None;
    };
    let courses = courses
        .into_iter()
        .map(|c| AutoCompleteOption::new(c.sigle.clone(), c.sigle + " - " + &c.name))
        .collect();

    let add_course = create_action(
        move |(sigle, set): &(String, WriteSignal<Vec<ReactiveCourse>>)| {
            let sigle = sigle.clone();
            let set = *set;
            async move {
                if let Ok(c) = get_course(sigle).await {
                    set.update(|s| {
                        if !s.iter().any(|react_c| react_c.sigle == c.sigle) {
                            s.push(c.into())
                        }
                    });
                    submit();
                }
            }
        },
    );

    let on_submit = move |sigle: String| {
        set_active_tab(sigle.clone());
        add_course.dispatch((sigle, set_selections));
    };

    Some(view! {
        <AutoComplete suggestion_list=courses placeholder="Cours" submit=on_submit id="course-submitter"/>
    })
}
