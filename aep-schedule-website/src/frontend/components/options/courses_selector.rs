use super::state::OptionState;
use crate::backend::routes::get_courses;
use crate::frontend::components::options::search::SearchCourse;
use aep_schedule_generator::data::group::Group;
use aep_schedule_generator::data::groups::Groups;
use leptos::*;
use phosphor_leptos::IconWeight;
use phosphor_leptos::Trash;

#[component]
pub fn Groups(groups: ReadSignal<Groups>, set_groups: WriteSignal<Groups>) -> impl IntoView {
    let row_class = |g: &Group| {
        let mut row_class = "row-container".to_string();
        if !g.initially_open {
            row_class.push_str(" closed-group");
        }
        row_class
    };
    view! {
        {move || groups.get().into_iter().enumerate().map(|(i, g)| {
                view!{

                    <div class=row_class(&groups.get_untracked()[i])>
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

    view! {
        <div>
            <Await
                future=|| get_courses()
                let:courses
            >
                <SearchCourse courses=courses.clone() set_selections/>
            </Await>
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
                }><Trash weight=IconWeight::Fill size="16px"/></button>
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
