use leptos::*;

#[component]
pub fn Schedule(#[prop(optional)] last_day: Option<u8>, children: Children) -> impl IntoView {
    const DAY_WEEK: [&str; 7] = [
        "Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Samedi", "Dimanche",
    ];
    const HOURS: [&str; 14] = [
        "8:30", "9:30", "10:30", "11:30", "12:30", "13:30", "14:30", "15:30", "16:30", "17:30",
        "18:30", "19:30", "20:30", "21:30",
    ];

    let day_week = match last_day {
        None => &DAY_WEEK[0..5],
        Some(last_day) => &DAY_WEEK[0..last_day as usize],
    };

    view! {
        <div class="schedule">
            <div class="days">
                <div></div>
                <div></div>
                {day_week.iter().map(|d| view!{<div class="day">{*d}</div>}).collect_view()}
            </div>
            <div class="content">
                {HOURS.map(|h| view!{<div class="time">{h}</div>})}
                <div class="filler-col"></div>
                {(3..(day_week.len()+2)).map(|i| view!{<div class="col" style={format!("grid-column:{i}")}></div>}).collect_view()}
                {(1..=HOURS.len()).map(|i| view!{<div class="row" style={format!("grid-row:{}/ span 2", 2 * i - 1)}></div>}).collect_view()}
                {children()}
            </div>
        </div>
    }
}
