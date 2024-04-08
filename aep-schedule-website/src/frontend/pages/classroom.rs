use crate::{
    backend::routes::{get_classroom, get_classrooms},
    frontend::components::common::{
        autocomplete::{AutoComplete, AutoCompleteOption},
        schedule::{Schedule, ScheduleEvent},
    },
};
use aep_schedule_generator::data::time::period::PeriodCourse;
use leptos::*;
use phosphor_leptos::WarningCircle;

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
        <div class="col-container classroom-page">
            <div class="warning-box">
                <WarningCircle size="5em"/>
                <span>
                    <span>"Cet horaire est construit à partir de l'horaire général des cours de Polytechnique Montréal. D'autres activités peuvent occuper un local. Pour connaître l'horaire complet d'un local ou le réserver: "</span>
                    <a href="https://www.polymtl.ca/renseignements-generaux/reserver-une-salle-ou-organiser-un-evenement">"Réserver une salle"</a>
                </span>
            </div>
            <Await
                // `future` provides the `Future` to be resolved
                future=|| get_classrooms()
                // the data is bound to whatever variable name you provide
                let:classrooms
            >
                {classrooms.as_ref().map(|classrooms| {
                    let classrooms = classrooms.iter().map(|c| AutoCompleteOption::new(c.to_string(), c.to_string())).collect();
                    view!{
                        <AutoComplete suggestion_list=classrooms placeholder="Local" class="input-item" submit=on_submit id="input-classroom"/>
                    }
                }).ok()}
            </Await>
            <Schedule last_day=5 col_height="0.6em">
                {move || periods.get().iter().enumerate().map(|(i, p)| view!{<PeriodEvent i period_course=p/>}).collect_view()}
            </Schedule>
        </div>
    }
}
