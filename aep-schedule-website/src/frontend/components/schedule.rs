use aep_schedule_generator::{
    algorithm::{schedule::Schedule, taken_course::TakenCourse},
    data::{
        group::Group,
        time::{calendar::Calendar, period::Period, week_number::WeekNumber},
    },
};
use leptos::{html::A, *};
use url::form_urlencoded;

#[component]
pub fn Course<'a>(course: &'a TakenCourse) -> impl IntoView {
    let lab_group = course
        .lab_group
        .as_ref()
        .map(|g| format!("L: {}", g.number));
    let theo_group = course
        .theo_group
        .as_ref()
        .map(|g| format!("C: {}", g.number));
    view! {
        <tr>
            <td>{course.sigle.to_string()}</td>
            <td>{course.name.to_string()}</td>
            <td>{lab_group}</td>
            <td>{theo_group}</td>
        </tr>
    }
}

fn style_p(period: &Period) -> String {
    let column = period.day as u8 + 3;
    let hour = 2 * (period.hours.starting_hour() - 6);
    let len = period.hours.len_hour();
    format!(
        "grid-column:{};grid-row:{} / span {};",
        column,
        hour,
        len * 2
    )
}

fn group_style(group: &Group, period: &Period) -> String {
    let mut class = "event".to_string();
    if group.conflict {
        class.push_str(" conflict");
    }
    match period.week_nb {
        WeekNumber::B1 => class.push_str(" b1"),
        WeekNumber::B2 => class.push_str(" b2"),
        _ => (),
    }
    class
}

#[component]
pub fn ScheduleComponent(schedule: Schedule, calendar: Calendar) -> impl IntoView {
    const HOURS: [&str; 14] = [
        "8:30", "9:30", "10:30", "11:30", "12:30", "13:30", "14:30", "15:30", "16:30", "17:30",
        "18:30", "19:30", "20:30", "21:30",
    ];
    const DAY_WEEK: [&str; 5] = ["Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi"];

    let schedule2 = schedule.clone();
    let (download, set_download) = create_signal("".to_string());
    let link: NodeRef<A> = create_node_ref();

    view! {
        <button on:click=move |_| {
            let ics = schedule2.generate_ics(&calendar);
            let url: String = form_urlencoded::byte_serialize(ics.as_bytes()).collect();
            set_download("data:text/plain;charset=utf-8,".to_string() + &url);
            link().unwrap().click();
        }>"Générer calendrier"</button>
        <a class="hidden" download="cours.ics" href=move || download.get() node_ref=link></a>
        <div class="schedule-container">
            <table class="cours">
                {schedule.courses.iter().map(|c| view!{<Course course=c/>}).collect_view()}
            </table>
            <div class="schedule">
                <div class="days">
                    <div></div>
                    <div></div>
                    {DAY_WEEK.map(|d| view!{<div class="day">{d}</div>})}
                </div>
                <div class="content">
                    {HOURS.into_iter().enumerate().map(|(i, h)| view!{<div class="time" style={format!("grid-row:{}", 2 * i + 2)}>{h}</div>}).collect_view()}
                    <div class="filler-col"></div>
                    {(3..(DAY_WEEK.len()+2)).map(|i| view!{<div class="col" style={format!("grid-column:{i}")}></div>}).collect_view()}
                    {(1..=HOURS.len()).map(|i| view!{<div class="row" style={format!("grid-row:{}/ span 2", 2 * i - 1)}></div>}).collect_view()}
                    {schedule.courses.iter().map(|c| {
                        view!{
                            {c.lab_group.as_ref().map(|g| Some(g.periods.iter().map(|p| {
                                view!{<div class={group_style(g, p)} style=style_p(p)><p>{c.sigle.to_string()}</p><p>"Lab. Gr: " {g.number} " " {p.week_nb.to_string()}</p></div>}
                            }).collect_view()))}
                            {c.theo_group.as_ref().map(|g| Some(g.periods.iter().map(|p| {
                                view!{<div class={group_style(g, p)} style=style_p(p)><p>{c.sigle.to_string()}</p><p>"Theo. Gr: " {g.number} " " {p.week_nb.to_string()}</p></div>}
                            }).collect_view()))}
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}
