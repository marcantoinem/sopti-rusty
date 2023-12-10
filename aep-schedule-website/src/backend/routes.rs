use leptos::server;
use leptos::ServerFnError;

#[server(GetSchedule, "/api")]
pub async fn get_schedules() -> Result<Vec<String>, ServerFnError> {
    use aep_schedule_generator::algorithm::schedules::Schedules;
    println!("Hello from the server");
    Ok(vec![])
}
