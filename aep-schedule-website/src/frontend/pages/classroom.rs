use crate::{
    backend::routes::{get_classroom, get_classrooms},
    frontend::components::common::{
        autocomplete::{AutoComplete, AutoCompleteOption},
        schedule::{Schedule, ScheduleEvent},
    },
};
use aep_schedule_generator::data::time::period::PeriodCourse;
use leptos::*;

#[component]
fn PeriodEvent<'a>(i: usize, period_course: &'a PeriodCourse) -> impl IntoView {
    let time = period_course.period.hours.to_string();
    let class = match i % 4 {
        0 => " color1",
        1 => " color2",
        2 => " color3",
        _ => " color4",
    };
    let sigle = period_course.sigle.to_string();

    view! {
        <ScheduleEvent period=&period_course.period class=class>
            <span>{sigle}</span>
            <span>{time}</span>
        </ScheduleEvent>
    }
}

#[component]
pub fn ClassRoomComponent() -> impl IntoView {
    let (periods, set_periods) = create_signal(vec![]);

    let change_classroom =
        create_action(|(room, set): &(String, WriteSignal<Vec<PeriodCourse>>)| {
            let set = *set;
            let room = room.clone();
            async move {
                if let Ok(periods) = get_classroom(room.into()).await {
                    set.set(periods);
                }
            }
        });

    let on_submit = move |sigle: String| change_classroom.dispatch((sigle, set_periods));
    view! {
        <Await
            // `future` provides the `Future` to be resolved
            future=|| get_classrooms()
            // the data is bound to whatever variable name you provide
            let:classrooms
        >
            {classrooms.as_ref().map(|classrooms| {
                let classrooms = classrooms.iter().map(|c| AutoCompleteOption::new(c.to_string(), c.to_string())).collect();
                view!{
                    <AutoComplete suggestion_list=classrooms placeholder="Locaux" class="input-item" submit=on_submit id="input-classroom"/>
                }
            }).ok()}
        </Await>
        <Schedule last_day=5>
            {move || periods.get().iter().enumerate().map(|(i, p)| view!{<PeriodEvent i period_course=p/>}).collect_view()}
        </Schedule>
    }
}
