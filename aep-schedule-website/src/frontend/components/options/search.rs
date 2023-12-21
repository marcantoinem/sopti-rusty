use super::state::ReactiveCourse;
use crate::backend::routes::get_course;
use aep_schedule_generator::data::course::CourseName;
use leptos::*;
use std::cmp;
use thaw::AutoComplete;
use thaw::AutoCompleteOption;

fn binary_search_start(
    courses: &[CourseName],
    sigle: String,
    mut submit: impl FnMut() -> (),
) -> &[CourseName] {
    let sigle = sigle.to_uppercase();
    let sigle = sigle.trim();
    let i = sigle.len();
    let bottom = courses.partition_point(|c| c.sigle[0..cmp::min(i, c.sigle.len())] < sigle[0..i]);
    let top = courses.partition_point(|c| c.sigle[0..cmp::min(i, c.sigle.len())] <= sigle[0..i]);
    if bottom < courses.len() && courses[bottom].sigle == sigle {
        submit();
        return courses;
    }
    &courses[bottom..top]
}

#[component]
pub fn SearchCourse(
    courses: Result<Vec<CourseName>, ServerFnError>,
    set_selections: WriteSignal<Vec<ReactiveCourse>>,
    set_active_tab: WriteSignal<String>,
) -> impl IntoView {
    let Ok(courses) = courses else {
        return None;
    };

    let add_course = create_action(
        |(c, set): &(RwSignal<String>, WriteSignal<Vec<ReactiveCourse>>)| {
            let sigle = c.get_untracked();
            let sigle = sigle.trim();
            let sigle = sigle.to_uppercase();
            c.set("".to_string());
            let set = set.clone();
            async move {
                if let Ok(c) = get_course(sigle).await {
                    set.update(|s| {
                        if !s.iter().any(|react_c| react_c.sigle == c.sigle) {
                            s.push(c.into())
                        }
                    });
                }
            }
        },
    );

    let sigle = create_rw_signal(String::new());
    let options = create_memo(move |_| {
        binary_search_start(&courses, sigle.get(), move || {
            let active_tab = sigle.get_untracked();
            let active_tab = active_tab.trim().to_uppercase();
            set_active_tab.set(active_tab);
            add_course.dispatch((sigle, set_selections))
        })
        .iter()
        .map(|c| AutoCompleteOption {
            label: format!("{} - {}", c.sigle, c.name),
            value: format!("{}", c.sigle),
        })
        .collect()
    });
    Some(view! {
        <AutoComplete value=sigle options placeholder="Cours" class="input-item"/>
    })
}
