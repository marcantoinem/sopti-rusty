use crate::frontend::components::common::schedule::{Schedule, ScheduleEvent};
use aep_schedule_generator::{
    algorithm::{schedule::Schedule, taken_course::TakenCourse},
    data::time::{calendar::Calendar, period::Period},
};
use leptos::{html::A, *};
use phosphor_leptos::{Download, IconWeight};

#[component]
pub fn Course<'a>(course: &'a TakenCourse) -> impl IntoView {
    let theo_group = course
        .theo_group
        .as_ref()
        .map(|g| format!("T: {}", g.number));
    let lab_group = course
        .lab_group
        .as_ref()
        .map(|g| format!("L: {}", g.number));
    view! {
        <tr>
            <td>{course.sigle.to_string()}</td>
            <td>{course.name.to_string()}</td>
            <td>{theo_group}</td>
            <td>{lab_group}</td>
        </tr>
    }
}

#[component]
fn PeriodEvent<'a>(
    i: usize,
    period: &'a Period,
    course: &'a TakenCourse,
    period_type: &'static str,
) -> impl IntoView {
    let location = period.hours.to_string() + " - " + period.room.as_str();
    let sigle = course.sigle.to_string() + " - " + period_type;
    let class = match i % 4 {
        0 => " color1",
        1 => " color2",
        2 => " color3",
        _ => " color4",
    };

    view! {
        <ScheduleEvent period=&period class=class>
            <span>{location}</span>
            <span>{sigle}</span>
        </ScheduleEvent>
    }
}

#[component]
fn CoursePeriods<'a>(i: usize, course: &'a TakenCourse) -> impl IntoView {
    let lab = course.lab_group.as_ref().map(|c| {
        c.periods
            .iter()
            .map(|p| {
                view! {<PeriodEvent i period=&p course=course period_type="L"/>}
            })
            .collect_view()
    });
    let theo = course.theo_group.as_ref().map(|c| {
        c.periods
            .iter()
            .map(|p| {
                view! {<PeriodEvent i period=&p course=course period_type="T"/>}
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
    let schedule2 = schedule.clone();
    let (download, set_download) = create_signal("".to_string());
    let link: NodeRef<A> = create_node_ref();

    view! {
        <a class="hidden" download="cours.ics" href=move || download.get() node_ref=link></a>
        <table class="cours">
            {schedule.taken_courses.iter().map(|c| view!{<Course course={c} />}).collect_view()}
        </table>
        <Schedule last_day=schedule.last_day>
            {schedule.taken_courses.iter().enumerate().map(|(i, c)| view!{<CoursePeriods i course=c />}).collect_view()}
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
