use super::state::OptionState;
use super::state::ReactiveCourse;
use crate::backend::routes::get_courses;
use crate::frontend::components::options::search::SearchCourse;
use aep_schedule_generator::data::groups::Groups;
use leptos::*;
use phosphor_leptos::IconWeight;
use phosphor_leptos::X;
use thaw::Checkbox;

#[component]
fn GroupsSettings(groups: Groups, open: Vec<RwSignal<bool>>) -> impl IntoView {
    view! {
        {groups.into_iter().enumerate().map(|(i, g)| {
                let open = open.clone();
                view!{
                    <div class="row-container" class=("closed-group", !g.open)>
                        <Checkbox value=open[i]>{g.number}</Checkbox>
                    </div>
                }
            }).collect_view()
        }
    }
}

#[component]
fn CourseTab(course: ReactiveCourse, active_tab: ReadSignal<String>) -> impl IntoView {
    view! {
        <div class="relative card tab tab-width" class=("hidden", {let sigle = course.sigle.to_string(); move || sigle.clone() != active_tab.get()})>
            <p>{course.name}</p>
            <div class="row-container row-evenly">
                <div>
                    <h3>"Théorie"</h3>
                    <GroupsSettings groups=course.theo_groups open=course.theo_open/>
                </div>
                <div>
                    <h3>"Laboratoire"</h3>
                    <GroupsSettings groups=course.lab_groups open=course.lab_open/>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn CoursesSelector(state: OptionState) -> impl IntoView {
    let (selections, set_selections) = state.selections;
    let (active_tab, set_active_tab) = create_signal("".to_string());
    view! {
        <Await
            future=|| get_courses()
            let:courses
        >
            <SearchCourse courses=courses.clone() set_selections set_active_tab/>
        </Await>
        <div class="row-container row-center tab-width">
            <For
                each=selections
                key=|c| c.sigle.clone()
                children=move |course| {
                    let sigle = course.sigle.to_string();
                    let add_hidden = move || sigle == active_tab.get();
                    let sigle = course.sigle.to_string();
                    view!{
                        <button class="tab-button" class=("tab-selected", add_hidden) id=&sigle on:click={
                            let sigle = sigle.clone();
                            move |_| set_active_tab.set(sigle.clone())
                        }>
                        {&sigle}
                        <button class="close" on:click={
                            let sigle = sigle.clone();
                            move |_| {
                                set_selections.update(|courses| courses.retain(|c| c.sigle.as_str() != sigle))
                            }
                        }><X weight=IconWeight::Regular size="16px"/></button>
                        </button>
                    }
                }
            />
        </div>
        <For
            each=selections
            key=|c| c.sigle.clone()
            children=move |course| {
                view!{
                    <CourseTab course active_tab/>
                }
            }
        />
    }
}
