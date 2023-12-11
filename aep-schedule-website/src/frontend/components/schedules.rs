use aep_schedule_generator::algorithm::schedule::Schedule;
use leptos::*;

#[component]
pub fn SchedulesComponent(read_signal: ReadSignal<Vec<(usize, Schedule)>>) -> impl IntoView {
    view! {
        <For
            each=read_signal
            key=|(i, _)| *i
            children=move |(id, s)| {
                view! {
                    <p>{s.courses.iter().map(|c| view!{<p>{c.name.to_string()}</p>}).collect_view()} "Test"</p>
                }
            }
        />
    }
}
