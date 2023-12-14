use super::state::OptionState;
use crate::backend::routes::get_course;
use crate::backend::routes::get_courses;
use crate::frontend::components::options::state::ReactiveCourse;
use aep_schedule_generator::data::groups::Groups;
use leptos::html::Input;
use leptos::*;

#[component]
pub fn Groups(groups: ReadSignal<Groups>, set_groups: WriteSignal<Groups>) -> impl IntoView {
    view! {
        {move || groups.get().into_iter().enumerate().map(|(i, g)| {
                    view!{
                        <div class="row-container">
                            <p>{move || g.number}</p>
                            <input type="checkbox" prop:checked={g.open} on:click=move |_| {
                                set_groups.update(|groups| groups[i].open = !groups[i].open);
                            }/>
                        </div>
                    }
            }).collect_view()
        }
    }
}

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
    let add_course = create_action(
        |(c, set): &(NodeRef<Input>, WriteSignal<Vec<ReactiveCourse>>)| {
            let sigle = c().unwrap().value();
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
            <div class="relative card tab">
                <p>{course.sigle.to_string()} " - " {course.name}</p>
                <button class="top-left" on:click=move |_| {
                    set_selections.update(|courses| courses.retain(|c| c.sigle != course.sigle.to_string()))
                }>"🗑️"</button>
                <input type="checkbox" name="accordion" class="accordion" checked/>
                <div class="row-container row-center tab-content">
                    <div>
                        <h3>"Théorie"</h3>
                        <Groups groups=course.theo_groups.0 set_groups=course.theo_groups.1/>
                    </div>
                    <div>
                        <h3>"Laboratoire"</h3>
                        <Groups groups=course.lab_groups.0 set_groups=course.lab_groups.1/>
                    </div>
                </div>
            </div>

        </For>
    }
}
