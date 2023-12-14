use crate::backend::routes::get_course;
use crate::backend::routes::get_courses;
use aep_schedule_generator::data::course::Course;
use leptos::html::Input;
use leptos::*;

use super::state::OptionState;

#[component]
pub fn CoursesSelector(state: OptionState) -> impl IntoView {
    let (selections, set_selections) = state.selections;
    let courses = create_resource(
        || (),
        |_| async move {
            match get_courses().await {
                Ok(v) => v,
                Err(_) => vec![],
            }
        },
    );

    let course_sigle: NodeRef<Input> = create_node_ref();
    let add_course = create_action(|(c, set): &(NodeRef<Input>, WriteSignal<Vec<Course>>)| {
        let sigle = c().unwrap().value();
        let set = set.clone();
        async move {
            if let Ok(c) = get_course(sigle).await {
                set.update(|s| {
                    if !s.contains(&c) {
                        s.push(c)
                    }
                });
            }
        }
    });

    view! {
        <div>
            <input type="text" list="courses" placeholder="Entrez le sigle ici" node_ref={course_sigle}/>
            <datalist id="courses">
                <Suspense fallback=move || view! {}>
                    {move || courses.get().map(|data| Some(
                        data.iter().map(|c| view!{<option value={c.sigle.to_string()}></option>}).collect_view()))}
                </Suspense>
            </datalist>
            <input type="submit" hidden/>
            <button on:click=move |_| {
                add_course.dispatch((course_sigle, set_selections))
            }>"+"</button>
        </div>
        <For
            each=selections
            key=|c| c.sigle.clone()
            let:course
        >
            <div class="relative card">
                <p>{course.sigle.to_string()}</p>
                <p>{course.name}</p>
                <button class="top-left" on:click=move |_| {
                    set_selections.update(|courses| courses.retain(|c| c.sigle != course.sigle.to_string()))
                }>"🗑️"</button>
            </div>
        </For>
    }
}
