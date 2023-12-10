use leptos::*;

#[server(GetSchedule, "/api")]
pub async fn get_schedules() -> Result<Vec<String>, ServerFnError> {
    use crate::backend::state::AppState;
    let courses = AppState::courses();
    Ok(vec![])
}
