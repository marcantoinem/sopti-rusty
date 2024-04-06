use crate::frontend::components::common::schedule::{Schedule, ScheduleEvent};
use aep_schedule_generator::{
    algorithm::{schedule::Schedule, taken_course::TakenCourse},
    data::time::{calendar::Calendar, period::Period},
};
use leptos::{html::A, *};
use phosphor_leptos::{Download, IconWeight};

#[component]
pub fn Course<'a>(course: &'a TakenCourse) -> impl IntoView {
    let lab_group = course
        .lab_group
        .as_ref()
        .map(|g| format!("L: {}", g.number));
    let theo_group = course
        .theo_group
        .as_ref()
        .map(|g| format!("C: {}", g.number));
    view! {
        <tr>
            <td>{course.sigle.to_string()}</td>
            <td>{course.name.to_string()}</td>
            <td>{lab_group}</td>
            <td>{theo_group}</td>
        </tr>
    }
}

#[component]
fn PeriodEvent<'a>(
    period: &'a Period,
    course: &'a TakenCourse,
    period_type: &'static str,
) -> impl IntoView {
    let location = period.hours.to_string() + " - " + period.room.as_str();
    let sigle = course.sigle.to_string() + " - " + period_type;
    view! {
        <ScheduleEvent period=&period>
            <span>{location}</span>
            <span>{sigle}</span>
        </ScheduleEvent>
    }
}

#[component]
fn CoursePeriods<'a>(course: &'a TakenCourse) -> impl IntoView {
    let lab = course.lab_group.as_ref().map(|c| {
        c.periods
            .iter()
            .map(|p| {
                view! {<PeriodEvent period=&p course=course period_type="Laboratoire"/>}
            })
            .collect_view()
    });
    let theo = course.theo_group.as_ref().map(|c| {
        c.periods
            .iter()
            .map(|p| {
                view! {<PeriodEvent period=&p course=course period_type="Théorie"/>}
            })
            .collect_view()
    });
    view! {
        {lab}
        {theo}
    }
}

#[component]
pub fn ScheduleComponent(schedule: Schedule, calendar: Calendar) -> impl IntoView {
    const HOURS: [&str; 14] = [
        "8:30", "9:30", "10:30", "11:30", "12:30", "13:30", "14:30", "15:30", "16:30", "17:30",
        "18:30", "19:30", "20:30", "21:30",
    ];
    const DAY_WEEK: [&str; 5] = ["Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi"];

    let schedule2 = schedule.clone();
    let (download, set_download) = create_signal("".to_string());
    let link: NodeRef<A> = create_node_ref();

    view! {
        <a class="hidden" download="cours.ics" href=move || download.get() node_ref=link></a>
        {schedule.taken_courses.iter().map(|c| view!{<Course course={c} />}).collect_view()}
        <Schedule>
            {schedule.taken_courses.iter().map(|c| view!{<CoursePeriods course=c />}).collect_view()}
        </Schedule>
        <button class="button-download" on:click=move |_| {
            let ics = schedule2.generate_ics(&calendar);
            let url = url_escape::encode_fragment(&ics);
            set_download("data:text/plain;charset=utf-8,".to_string() + &url);
            link().unwrap().click();
        }>
            <Download weight=IconWeight::Regular size="3vh"/>
            <span>"Télécharger le calendrier de cet horaire"</span>
        </button>
    }
}
