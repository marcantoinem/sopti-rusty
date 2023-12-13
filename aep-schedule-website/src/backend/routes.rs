use aep_schedule_generator::data::courses::SchedulesOptions;
use aep_schedule_generator::{algorithm::schedule::Schedule, data::course::CourseName};
use leptos::*;

#[server(GetSchedule, "/api")]
pub async fn get_schedules(
    schedule_options: SchedulesOptions,
) -> Result<Vec<Schedule>, ServerFnError> {
    use crate::backend::state::AppState;
    let courses = AppState::courses().await?;
    let schedules = courses
        .read()
        .await
        .get_schedules(schedule_options)
        .into_sorted_vec();
    Ok(schedules)
}

#[server(GetCoursesName, "/api")]
pub async fn get_courses() -> Result<Vec<CourseName>, ServerFnError> {
    use crate::backend::state::AppState;
    let courses = AppState::courses().await?;
    let courses = courses.read().await;
    Ok(courses.get_courses_name())
}
