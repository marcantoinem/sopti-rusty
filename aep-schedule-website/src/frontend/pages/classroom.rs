use crate::frontend::components::icons::warning_circle::WarningCircle;
use crate::{
    backend::routes::{get_classroom, get_classrooms},
    frontend::components::common::{
        autocomplete::{AutoComplete, AutoCompleteOption},
        schedule::{Schedule, ScheduleEvent},
    },
};
use aep_schedule_generator::data::time::{period::PeriodCourse, week_number::WeekNumber};
use leptos::*;

#[component]
fn PeriodEvent<'a>(i: usize, period_course: &'a PeriodCourse) -> impl IntoView {
    let time = period_course.period.hours.to_string();
    let sigle = period_course.sigle.to_string();

    let mut class = match i % 4 {
        0 => " color1".to_string(),
        1 => " color2".to_string(),
        2 => " color3".to_string(),
        _ => " color4".to_string(),
    };
    match period_course.period.week_nb {
        WeekNumber::B1 => class.push_str(" b1"),
        WeekNumber::B2 => class.push_str(" b2"),
        _ => (),
    }

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
        <section class="flex flex-col w-full justify-between items-center p-4">
            <div class="warning-box">
                <WarningCircle size="5em"/>
                <span>
                    <span>"Cet horaire est construit à partir de l'horaire général des cours de Polytechnique Montréal. D'autres activités peuvent occuper un local. Pour connaître l'horaire complet d'un local ou le réserver: "</span>
                    <a href="https://www.polymtl.ca/renseignements-generaux/reserver-une-salle-ou-organiser-un-evenement">"Réserver une salle"</a>
                </span>
            </div>
            <Await
                future=|| get_classrooms()
                let:classrooms
            >
                {classrooms.as_ref().map(|classrooms| {
                    let classrooms = classrooms.iter().map(|c| AutoCompleteOption::new(c.to_string(), c.to_string())).collect();
                    view!{
                        <AutoComplete suggestion_list=classrooms placeholder="Local" class="w-96" submit=on_submit id="input-classroom"/>
                    }
                }).ok()}
            </Await>
            <Schedule last_day=5 col_height="0.6em">
                {move || periods.get().iter().enumerate().map(|(i, p)| view!{<PeriodEvent i period_course=p/>}).collect_view()}
            </Schedule>
        </section>
    }
}
