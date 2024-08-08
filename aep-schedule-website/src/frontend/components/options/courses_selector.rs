use crate::backend::routes::get_courses;
use crate::frontend::components::common::tab::Tab;
use crate::frontend::components::icons::bell_ringing::BellRinging;
use crate::frontend::components::icons::calendar_x::CalendarX;
use crate::frontend::components::icons::x::X;
use crate::frontend::components::icons::IconWeight;
use crate::frontend::components::options::personal::PersonalTimeSelector;
use crate::frontend::components::options::search::SearchCourse;
use crate::frontend::pages::generator::SetModal;
use crate::frontend::state::reactive_course::ReactiveCourse;
use crate::frontend::state::reactive_course::ReactiveCourseType;
use crate::frontend::state::OptionState;
use aep_schedule_generator::data::group::Group;
use aep_schedule_generator::data::group_sigle::GroupType;
use aep_schedule_generator::data::group_sigle::SigleGroup;
use aep_schedule_generator::data::groups::Groups;
use compact_str::CompactString;
use leptos::*;

#[component]
fn GroupsChips<F>(
    open: RwSignal<bool>,
    group: Group,
    course_sigle: CompactString,
    group_type: GroupType,
    submit: F,
) -> impl IntoView
where
    F: Fn() + Copy + 'static,
{
    let set_modal = SetModal::from_context();

    view! {
        <div on:pointerdown=move |_| {
                open.update(|b| *b = !*b);
                submit();
            }
            class="gap-2 cursor-pointer items-center py-1.5 px-3 rounded-lg flex"
            class=("bg-green-500", move || {open.get()})
            class=("bg-red-500", move || {!open.get()})
        >
            <span class="text-lg text-bold text-sans">{group.number.to_string()}</span>
            <div class="flex flex-col justify-between w-full">
                {group.periods.iter().map(|p| {
                    view!{
                        <div class="flex group-text w-full justify-between">
                            <span>{p.day.to_string()}</span>
                            <span class="period-group">{p.hours.to_string()}</span>
                        </div>
                    }
                }).collect_view()}
            </div>
            //{match group.open {
            //    false => Some(view !{
            //        <div on:pointerdown=move |ev| {
            //            ev.stop_propagation();
            //            let sigle_group = SigleGroup::new(course_sigle.clone(), group_type, group.number);
            //            set_modal.set(Some(sigle_group));
            //            }>
            //            <BellRinging size="1em"/>
            //        </div>
            //    }),
            //    true => None,
            //}}
        </div>
    }
}

#[component]
fn GroupsSettings<F>(
    groups: Groups,
    open: Vec<RwSignal<bool>>,
    course_sigle: CompactString,
    group_type: GroupType,
    submit: F,
) -> impl IntoView
where
    F: Fn() + Copy + 'static,
{
    view! {
        {groups.into_iter().enumerate().map(|(i, group)| {
                let open = open[i];
                view! {
                    <GroupsChips open group course_sigle=course_sigle.clone() group_type submit/>
                }
            }).collect_view()
        }
    }
}

#[component]
fn CourseTab<F>(course: ReactiveCourse, active_tab: ReadSignal<String>, submit: F) -> impl IntoView
where
    F: Fn() + Copy + 'static,
{
    let course_sigle = course.sigle.clone();
    view! {
        <Tab active_tab tab_id=course.sigle.to_string()>
            <p>{course.name}</p>
            <div class="flex justify-around">
                {
                    match course.course_type {
                        ReactiveCourseType::TheoOnly { theo_open, theo_groups } => {
                            let groups = theo_groups;
                            view!{
                                <div class="flex gap-2 flex-col pb-2 max-h-[26rem] overflow-y-auto">
                                    <h3>"Théorie"</h3>
                                    <GroupsSettings groups open=theo_open course_sigle group_type=GroupType::TheoGroup submit/>
                                </div>
                            }.into_view()
                        },
                        ReactiveCourseType::LabOnly { lab_open, lab_groups } => {
                            let groups = lab_groups;
                            view!{
                                <div class="flex gap-2 flex-col pb-2 overflow-y-auto">
                                    <h3>"Laboratoire"</h3>
                                    <GroupsSettings groups open=lab_open course_sigle=course_sigle.clone() group_type=GroupType::LabGroup submit/>
                                </div>
                            }.into_view()
                        },
                        ReactiveCourseType::Both { theo_open, theo_groups, lab_open, lab_groups } => {
                            let theo_groups = theo_groups;
                            let lab_groups = lab_groups;
                            view!{
                                <div class="flex gap-2 flex-col pb-2 overflow-y-auto">
                                    <h3>"Théorie"</h3>
                                    <GroupsSettings groups=theo_groups open=theo_open course_sigle=course_sigle.clone() group_type=GroupType::TheoGroup submit/>
                                </div>
                                <div class="vertical-bar"></div>
                                <div class="flex gap-2 flex-col pb-2 overflow-y-auto">
                                    <h3>"Laboratoire"</h3>
                                    <GroupsSettings groups=lab_groups open=lab_open course_sigle=course_sigle.clone() group_type=GroupType::LabGroup submit/>
                                </div>
                            }.into_view()
                        },
                        ReactiveCourseType::Linked { both_open, theo_groups, lab_groups } => {
                            let groups = theo_groups.merge(lab_groups);
                            view!{
                                <div class="flex gap-2 flex-col pb-2 overflow-y-auto">
                                    <h3>"Théorie et laboratoire lié"</h3>
                                    <GroupsSettings groups open=both_open course_sigle=course_sigle group_type=GroupType::LabGroup submit/>
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
    let (active_tab, set_active_tab) = create_signal("".to_string());

    let action_courses = state.action_courses;

    view! {
        <Await
            future=get_courses
            let:courses
        >
            <SearchCourse courses=courses.clone() action_courses set_active_tab/>
        </Await>
        <div class="flex w-full flex-wrap gap-1">
            <button class="flex items-center py-1 px-2 rounded-xl bg-amber-500 text-black transition" class=("opacity-75", move || active_tab.get() != "") id="personal" on:pointerdown={
                move |_| set_active_tab.set("".to_string())
            }>
                <CalendarX weight=IconWeight::Regular size="16px"/>
                {"Horaire personnel"}
            </button>
            <For
                each=move || {action_courses.value().get().unwrap_or_default()}
                key=|c| c.sigle.clone()
                children=move |course| {
                    let sigle = course.sigle.to_string();
                    let add_hidden = move || sigle != active_tab.get();
                    let sigle = course.sigle.to_string();
                    view!{
                        <button class="flex items-center py-1 px-2 rounded-xl bg-amber-500 text-black transition" class=("opacity-75", add_hidden) id=&sigle on:pointerdown={
                            let sigle = sigle.clone();
                            move |_| set_active_tab.set(sigle.clone())
                        }>
                        {&sigle}
                        <button class="close" on:pointerdown={
                            let sigle = sigle.clone();
                            move |_| {
                                action_courses.value().update(|courses| {
                                    if let Some(courses) = courses {
                                        courses.retain(|c| c.sigle.as_str() != sigle);
                                    }}
                                );
                                state.stored_courses.update_value(|courses| {
                                    courses.retain(|c| c.sigle.as_str() != sigle);
                                });
                                submit();
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
            each=move || {action_courses.value().get().unwrap_or_default()}
            key=|c| c.sigle.clone()
            let:course
        >
            <CourseTab course active_tab submit/>
        </For>
    }
}
