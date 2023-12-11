use aep_schedule_generator::algorithm::schedule::Schedule;
use aep_schedule_generator::data::courses::SchedulesOptions;
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
