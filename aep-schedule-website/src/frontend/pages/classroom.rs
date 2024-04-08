use crate::{
    backend::routes::{get_classroom, get_classrooms},
    frontend::components::common::{
        autocomplete::{AutoComplete, AutoCompleteOption},
        schedule::{Schedule, ScheduleEvent},
    },
};
use aep_schedule_generator::data::time::period::Period;
use leptos::*;

#[component]
fn PeriodEvent<'a>(i: usize, period: &'a Period) -> impl IntoView {
    let time = period.hours.to_string();
    let class = match i % 4 {
        0 => " color1",
        1 => " color2",
        2 => " color3",
        _ => " color4",
    };

    view! {
        <ScheduleEvent period=&period class=class>
            <span>{time}</span>
        </ScheduleEvent>
    }
}

#[component]
pub fn ClassRoomComponent() -> impl IntoView {
    let (periods, set_periods) = create_signal(vec![]);

    let change_classroom = create_action(|(room, set): &(String, WriteSignal<Vec<Period>>)| {
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
            {move || periods.get().iter().enumerate().map(|(i, p)| view!{<PeriodEvent i period=p/>}).collect_view()}
        </Schedule>
    }
}
