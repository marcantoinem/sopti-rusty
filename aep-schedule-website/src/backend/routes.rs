use aep_schedule_generator::algorithm::{generation::SchedulesOptions, schedule::Schedule};
use aep_schedule_generator::data::course::{Course, CourseName};
use leptos::*;

#[server(GetCoursesName, "/api", "GetJson")]
pub async fn get_courses() -> Result<Vec<CourseName>, ServerFnError> {
    use crate::backend::state::AppState;
    let courses = AppState::courses().await?;
    let courses = courses.read().await;
    Ok(courses.get_courses_name())
}

#[server(GetCourse, "/api", "GetJson")]
pub async fn get_course(sigle: String) -> Result<Course, ServerFnError> {
    use crate::backend::state::AppState;
    let courses = AppState::courses().await?;
    let courses = courses.read().await;
    match courses.get_course(&sigle) {
        Some(s) => Ok(s),
        None => Err(ServerFnError::ServerError("Invalid sigle".to_string())),
    }
}
