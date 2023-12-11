use aep_schedule_generator::algorithm::{schedule::Schedule, taken_course::TakenCourse};
use leptos::*;

#[component]
pub fn Course<'a>(course: &'a TakenCourse) -> impl IntoView {
    let lab_group = course
        .lab_group
        .as_ref()
        .and_then(|g| Some(format!("L: {}", g.number)));
    let theo_group = course
        .theo_group
        .as_ref()
        .and_then(|g| Some(format!("C: {}", g.number)));
    view! {
        <h2>{course.sigle.to_string()}</h2>
        <h3>{course.name.to_string()}</h3>
        <p>{lab_group}</p>
        <p>{theo_group}</p>
    }
}

#[component]
pub fn ScheduleComponent(schedule: Schedule) -> impl IntoView {
    view! {
        <div class="schedule-container">
            {schedule.courses.iter().map(|c| view!{<Course course=c/>}).collect_view()}
        </div>
    }
}
