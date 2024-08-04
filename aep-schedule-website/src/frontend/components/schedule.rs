use std::rc::Rc;

use crate::frontend::components::common::schedule::{Schedule, ScheduleEvent};
use crate::frontend::components::icons::download::Download;
use crate::frontend::components::icons::IconWeight;
use aep_schedule_generator::{
    algorithm::{
        schedule::Schedule,
        taken_course::{TakenCourse, TakenCourseType},
    },
    data::time::{calendar::Calendar, period::Period, week_number::WeekNumber},
};
use leptos::{html::A, *};

#[component]
pub fn Course<'a>(i: usize, course: &'a TakenCourse) -> impl IntoView {
    let theo_group = course.theo_group().map(|g| format!("T: {}", g.number));
    let lab_group = course.lab_group().map(|g| format!("L: {}", g.number));
    let color_box = match i % 8 {
        0 => "w-5 h-5 color1",
        1 => "w-5 h-5 color2",
        2 => "w-5 h-5 color3",
        3 => "w-5 h-5 color4",
        4 => "w-5 h-5 color5",
        5 => "w-5 h-5 color6",
        6 => "w-5 h-5 color7",
        _ => "w-5 h-5 color8",
    };

    view! {
        <tr>
            <td class="flex items-center gap-1">
                <div class=color_box></div>
                <span>{course.sigle.to_string()}</span>
            </td>
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
    let mut class = match i % 8 {
        0 => " color1".to_string(),
        1 => " color2".to_string(),
        2 => " color3".to_string(),
        3 => " color4".to_string(),
        4 => " color5".to_string(),
        5 => " color6".to_string(),
        6 => " color7".to_string(),
        _ => " color8".to_string(),
    };
    match period.week_nb {
        WeekNumber::B1 => class.push_str(" b1"),
        WeekNumber::B2 => class.push_str(" b2"),
        _ => (),
    }

    view! {
        <ScheduleEvent period=&period class=class>
            <span>{location}</span>
            <span>{sigle}</span>
        </ScheduleEvent>
    }
}

#[component]
fn CoursePeriods<'a>(i: usize, course: &'a TakenCourse) -> impl IntoView {
    match &course.taken_course_type {
        TakenCourseType::TheoOnly { theo_group } => theo_group
            .periods
            .iter()
            .map(|p| {
                view! {<PeriodEvent i period=&p course=course period_type="T"/>}
            })
            .collect_view(),
        TakenCourseType::LabOnly { lab_group } => lab_group
            .periods
            .iter()
            .map(|p| {
                view! {<PeriodEvent i period=&p course=course period_type="L"/>}
            })
            .collect_view(),
        TakenCourseType::Both {
            theo_group,
            lab_group,
        }
        | TakenCourseType::Linked {
            theo_group,
            lab_group,
        } => view! {
            {
                theo_group.periods.iter().map(|p| {
                    view! {<PeriodEvent i period=&p course=course period_type="T"/>}
                }).collect_view()
            }
            {
                lab_group.periods.iter().map(|p| {
                    view! {<PeriodEvent i period=&p course=course period_type="L"/>}
                }).collect_view()
            }
        }
        .into_view(),
    }
}

#[component]
pub fn ScheduleComponent(schedule: Schedule, calendar: Rc<Calendar>) -> impl IntoView {
    let schedule2 = schedule.clone();
    let (download, set_download) = create_signal("".to_string());
    let link: NodeRef<A> = create_node_ref();

    view! {
        <div class="flex flex-col w-full items-center">
            <a class="hidden" download="cours.ics" href=move || download.get() node_ref=link></a>
            <table class="cours">
                {schedule.taken_courses.iter().enumerate().map(|(i, c)| view!{<Course i course={c} />}).collect_view()}
            </table>
            <Schedule last_day=schedule.last_day>
                {schedule.taken_courses.iter().enumerate().map(|(i, c)| view!{<CoursePeriods i course=c />}).collect_view()}
            </Schedule>
            //<button class="button-download flex" on:mousedown=move |_| {
            //    let ics = schedule2.generate_ics(&calendar);
            //    let url = url_escape::encode_fragment(&ics);
            //    set_download("data:text/plain;charset=utf-8,".to_string() + &url);
            //    link().unwrap().click();
            //}>
            //    <Download weight=IconWeight::Regular size="3vh"/>
            //    <span>"Télécharger le calendrier de cet horaire"</span>
            //</button>
        </div>
    }
}
