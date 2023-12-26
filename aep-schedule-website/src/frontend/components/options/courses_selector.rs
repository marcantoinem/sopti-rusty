use super::state::OptionState;
use super::state::ReactiveCourse;
use crate::backend::routes::get_courses;
use crate::frontend::components::common::checkbox::CheckboxChip;
use crate::frontend::components::common::tab::Tab;
use crate::frontend::components::options::search::SearchCourse;
use aep_schedule_generator::data::groups::Groups;
use leptos::*;
use phosphor_leptos::IconWeight;
use phosphor_leptos::X;

#[component]
fn GroupsSettings(groups: Groups, open: Vec<RwSignal<bool>>) -> impl IntoView {
    view! {
        {groups.into_iter().enumerate().map(|(i, g)| {
                let open = open[i];
                let open_style = if g.open {"group-chip"} else {"group-chip closed-group"};
                view!{
                    <CheckboxChip value=open class=open_style>
                        <span>{g.number}</span> <div class="col-container group-text-col">{g.periods.iter().map(|p| {
                            view!{<div class="row-container group-text">
                                <span>{p.day.to_string()}</span>
                                <span class="period-group">{p.hours.to_string()}</span>
                            </div>}
                        }).collect_view()}</div>
                    </CheckboxChip>
                }
            }).collect_view()
        }
    }
}

#[component]
fn CourseTab(course: ReactiveCourse, active_tab: ReadSignal<String>) -> impl IntoView {
    view! {
        <Tab active_tab tab_id=course.sigle.to_string()>
            <p>{course.name}</p>
            <div class="row-container row-evenly">
                <div>
                    <h3>"Théorie"</h3>
                    <GroupsSettings groups=course.theo_groups open=course.theo_open/>
                </div>
                <div class="vertical-bar">
                </div>
                <div>
                    <h3>"Laboratoire"</h3>
                    <GroupsSettings groups=course.lab_groups open=course.lab_open/>
                </div>
            </div>
        </Tab>
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
                        <button class="tab-button chips" class=("tab-selected", add_hidden) id=&sigle on:click={
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
