use super::state::OptionState;
use super::state::ReactiveCourse;
use crate::backend::routes::get_courses;
use crate::frontend::components::common::checkbox::CheckboxChip;
use crate::frontend::components::common::tab::Tab;
use crate::frontend::components::icons::calendar_x::CalendarX;
use crate::frontend::components::icons::x::X;
use crate::frontend::components::icons::IconWeight;
use crate::frontend::components::options::personal::PersonalTimeSelector;
use crate::frontend::components::options::search::SearchCourse;
use crate::frontend::components::options::state::ReactiveCourseType;
use aep_schedule_generator::data::time::period::Period;
use leptos::*;

#[component]
fn GroupsSettings(groups: Vec<(bool, Vec<Period>)>, open: Vec<RwSignal<bool>>) -> impl IntoView {
    view! {
        {groups.into_iter().enumerate().map(|(i, (initially_open, periods))| {
                let open_style = if initially_open {"group-chip"} else {"group-chip closed-group"};
                let open = open[i];
                view!{
                    <CheckboxChip value=open class=open_style>
                        <span>{i + 1}</span>
                        <div class="col-container group-text-col">
                            {periods.iter().map(|p| {
                                view!{
                                    <div class="row-container group-text">
                                        <span>{p.day.to_string()}</span>
                                        <span class="period-group">{p.hours.to_string()}</span>
                                    </div>
                                }
                            }).collect_view()}
                        </div>
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
                {
                    match course.course_type {
                        ReactiveCourseType::TheoOnly { theo_open, theo_groups } => {
                            let groups = theo_groups.into_iter().map(|g| (g.open, g.periods)).collect();
                            view!{
                                <div>
                                    <h3>"Théorie"</h3>
                                    <GroupsSettings groups open=theo_open/>
                                </div>
                            }.into_view()
                        },
                        ReactiveCourseType::LabOnly { lab_open, lab_groups } => {
                            let groups = lab_groups.into_iter().map(|g| (g.open, g.periods)).collect();
                            view!{
                                <div>
                                    <h3>"Laboratoire"</h3>
                                    <GroupsSettings groups open=lab_open/>
                                </div>
                            }.into_view()
                        },
                        ReactiveCourseType::Both { theo_open, theo_groups, lab_open, lab_groups } => {
                            let theo_groups = theo_groups.into_iter().map(|g| (g.open, g.periods)).collect();
                            let lab_groups = lab_groups.into_iter().map(|g| (g.open, g.periods)).collect();
                            view!{
                                <div>
                                    <h3>"Théorie"</h3>
                                    <GroupsSettings groups=theo_groups open=theo_open/>
                                </div>
                                <div class="vertical-bar"></div>
                                <div>
                                    <h3>"Laboratoire"</h3>
                                    <GroupsSettings groups=lab_groups open=lab_open/>
                                </div>
                            }.into_view()
                        },
                        ReactiveCourseType::Linked { both_open, theo_groups, lab_groups } => {
                            let groups = theo_groups.into_iter().zip(lab_groups.into_iter()).map(|(g1, mut g2)| {
                                let mut periods = g1.periods;
                                periods.append(&mut g2.periods);
                                (g1.open, periods)
                            }).collect();
                            view!{
                                <div>
                                    <h3>"Théorie et laboratoire lié"</h3>
                                    <GroupsSettings groups open=both_open/>
                                </div>
                            }.into_view()
                        },
                    }
                }
            </div>
        </Tab>
    }
}

#[component]
pub fn CoursesSelector<F>(state: OptionState, submit: F) -> impl IntoView
where
    F: Fn() + Copy + 'static,
{
    let (selections, set_selections) = state.selections;
    let (active_tab, set_active_tab) = create_signal("".to_string());
    view! {
        <Await
            future=get_courses
            let:courses
        >
            <SearchCourse courses=courses.clone() set_selections set_active_tab/>
        </Await>
        <div class="row-container tab-width">
            <button class="tab-button chips" class=("tab-selected", move || active_tab.get() == "") id="personal" on:click={
                move |_| set_active_tab.set("".to_string())
            }>
                <CalendarX weight=IconWeight::Regular size="16px"/>
                {"Horaire personnel"}
            </button>
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
        <Tab active_tab tab_id="".to_string()>
            <PersonalTimeSelector week=state.week submit></PersonalTimeSelector>
        </Tab>
        <For
            each=selections
            key=|c| c.sigle.clone()
            let:course
        >
            <CourseTab course active_tab/>
        </For>
    }
}
