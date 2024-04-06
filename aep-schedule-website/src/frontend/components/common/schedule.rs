use aep_schedule_generator::data::time::period::Period;
use leptos::*;
use std::array;

#[component]
pub fn Schedule(#[prop(optional)] last_day: Option<u8>, children: Children) -> impl IntoView {
    const DAY_WEEK: [&str; 7] = [
        "Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Samedi", "Dimanche",
    ];
    let hours: [String; 14] = array::from_fn(|i| format!("{}:30", i + 8));

    let day_week = match last_day {
        None => &DAY_WEEK[0..5],
        Some(last_day) => &DAY_WEEK[0..last_day as usize],
    };

    view! {
        <div class="schedule">
            <div class="days" style={format!("grid-template-columns:2em 10px repeat({}, 1fr)", day_week.len())}>
                <div></div>
                <div></div>
                {day_week.iter().map(|d| view!{<div class="day">{*d}</div>}).collect_view()}
            </div>
            <div class="content" style={format!("grid-template-columns:2em 10px repeat({}, 1fr)", day_week.len())}>
                {hours.clone().into_iter().enumerate().map(|(i, h)| view!{<div class="time" style={format!("grid-row:{}", 4 * (i + 1))}>{h}</div>}).collect_view()}
                <div class="filler-col"></div>
                {(3..(day_week.len()+2)).map(|i| view!{<div class="col" style={format!("grid-column:{i}")}></div>}).collect_view()}
                {(1..=hours.len()).map(|i| view!{<div class="row" style={format!("grid-row:{}/ span 2", 4 * i - 1)}></div>}).collect_view()}
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn ScheduleEvent<'a>(period: &'a Period, children: Children) -> impl IntoView {
    let column = period.day as u8 + 3;
    let hour = period.hours.start() + 3;
    let len = period.hours.len_hour();
    let style = format!(
        "grid-column:{};grid-row:{} / span {};",
        column,
        hour,
        len * 4
    );
    view! {
        <div style={style} class="event">
            {children()}
        </div>
    }
}
