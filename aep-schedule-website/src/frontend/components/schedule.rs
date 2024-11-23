use std::sync::Arc;

use crate::frontend::components::common::schedule::{Schedule, ScheduleEvent};
use crate::frontend::components::icons::download::Download;
use crate::frontend::components::icons::IconWeight;
use aep_schedule_generator::icalendar::calendar::Calendar;
use aep_schedule_generator::{
    algorithm::{
        schedule::Schedule,
        taken_course::{TakenCourse, TakenCourseType},
    },
    data::time::{period::Period, week_number::WeekNumber},
};
use leptos::prelude::*;

#[component]
pub fn Course(i: usize, course: TakenCourse) -> impl IntoView {
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
fn PeriodEvent(
    i: usize,
    period: Period,
    course: Arc<TakenCourse>,
    period_type: &'static str,
) -> impl IntoView {
    let mut location = period.hours.to_string() + " - " + period.room.as_str();
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
        WeekNumber::B1 => {
            class.push_str(" b1");
            location.push_str(" B1");
        }
        WeekNumber::B2 => {
            class.push_str(" b2");
            location.push_str(" B2");
        }
        _ => (),
    }

    view! {
        <ScheduleEvent period=period class=class>
            <span>{location}</span>
            <span>{sigle}</span>
        </ScheduleEvent>
    }
}

#[component]
fn CoursePeriods(i: usize, course: TakenCourse) -> impl IntoView {
    let taken_course_type = course.taken_course_type.clone();
    let course = Arc::new(course);
    match taken_course_type {
        TakenCourseType::TheoOnly { theo_group } => theo_group
            .periods
            .into_iter()
            .map(|p| {
                let course = Arc::clone(&course);
                view! {<PeriodEvent i period=p course period_type="T"/>}
            })
            .collect_view()
            .into_any(),
        TakenCourseType::LabOnly { lab_group } => lab_group
            .periods
            .into_iter()
            .map(|p| {
                let course = Arc::clone(&course);
                view! {<PeriodEvent i period=p course period_type="L"/>}
            })
            .collect_view()
            .into_any(),
        TakenCourseType::Both {
            theo_group,
            lab_group,
        }
        | TakenCourseType::Linked {
            theo_group,
            lab_group,
        } => view! {
            {
                theo_group.periods.into_iter().map(|p| {
                    let course = Arc::clone(&course);
                    view! {<PeriodEvent i period=p course period_type="T"/>}
                }).collect_view()
            }
            {
                lab_group.periods.into_iter().map(|p| {
                    let course = Arc::clone(&course);
                    view! {<PeriodEvent i period=p course period_type="L"/>}
                }).collect_view()
            }
        }
        .into_any(),
    }
}

#[component]
pub fn ScheduleComponent(schedule: Schedule, calendar: Arc<Calendar>) -> impl IntoView {
    let courses = schedule.taken_courses.clone();
    let courses2 = schedule.taken_courses.clone();
    let schedule2 = schedule.clone();
    let (download, set_download) = signal("".to_string());
    let link = NodeRef::new();

    view! {
        <div class="flex flex-col w-full items-center card p-2">
            <a class="hidden" download="cours.ics" href=move || download.get() node_ref=link></a>
            <table class="cours">
                {courses.into_iter().enumerate().map(|(i, c)| view!{<Course i course={c} />}).collect_view()}
            </table>
            <Schedule last_day=schedule.last_day>
                {courses2.into_iter().enumerate().map(|(i, c)| view!{<CoursePeriods i course=c />}).collect_view()}
            </Schedule>
            <button class="button-download flex" on:pointerdown=move |_| {
               let ics = calendar.generate_ics(&schedule2);
               let url = url_escape::encode_fragment(&ics);
               set_download("data:text/plain;charset=utf-8,".to_string() + &url);
               link.get().unwrap().click();
            }>
               <Download weight=IconWeight::Regular size="3vh"/>
               <span>"Télécharger le calendrier de cet horaire"</span>
            </button>
        </div>
    }
}
