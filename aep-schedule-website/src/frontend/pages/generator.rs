use crate::frontend::components::icons::{caret_double_right::CaretDoubleRight, IconWeight};
use crate::frontend::components::notifications::Notifications;
use crate::frontend::components::options::state::OptionState;
use crate::frontend::components::{options::form::OptionsForms, schedules::SchedulesComponent};
use aep_schedule_generator::algorithm::generation::SchedulesOptions;
use aep_schedule_generator::data::group_sigle::SigleGroup;
use leptos::*;

#[derive(Clone, Copy)]
pub struct SetModal(pub WriteSignal<Option<SigleGroup>>);

#[derive(Clone, Copy)]
pub struct FirstGenerationDone(pub RwSignal<bool>);

#[component]
pub fn GeneratorPage() -> impl IntoView {
    let (hide, set_hide) = create_signal(false);
    let first_generation_done = create_rw_signal(false);

    let (step, set_step) = create_signal(1);

    // Creates a reactive value to update the button
    let action = create_action(move |s: &SchedulesOptions| {
        let mut s = s.clone();
        set_hide(true);
        s.apply_personal_schedule();
        async move { s.get_schedules().into_sorted_vec() }
    });

    let (modal, set_modal) = create_signal(None);

    let state = OptionState::default();

    let section_error = create_rw_signal("".to_string());
    let personal_error = create_rw_signal("".to_string());

    let validate = move |state: OptionState| {
        let mut options: SchedulesOptions = (&state).into();
        if options.courses_to_take.is_empty() {
            set_step.set(1);
            return;
        }
        let mut impossible_courses = options.get_impossible_course().into_iter();
        if let Some(first_impossible_course) = impossible_courses.next() {
            let mut error = format!("Les sections des/du cours {}", first_impossible_course);
            for impossible_course in impossible_courses {
                error.push_str(", ");
                error.push_str(&impossible_course);
            }
            error.push_str(" sont toutes fermées.");
            section_error.set(error);
            set_step.set(2);
            return;
        }
        section_error.set("".to_string());
        options.apply_personal_schedule();
        let mut impossible_courses = options.get_impossible_course().into_iter();
        if let Some(first_impossible_course) = impossible_courses.next() {
            let mut error = format!("Les sections des/du cours {}", first_impossible_course);
            for impossible_course in impossible_courses {
                error.push_str(", ");
                error.push_str(&impossible_course);
            }
            error.push_str(" sont en conflits avec les heures libres sélectionnées.");
            personal_error.set(error);
            set_step.set(3);
            return;
        }
        personal_error.set("".to_string());
        set_step.set(5);
    };

    provide_context(state);
    provide_context(SetModal(set_modal));
    provide_context(FirstGenerationDone(first_generation_done));

    view! {
        <aside class="left-panel" class=("hide-left-panel", hide)>
            <OptionsForms action validate step/>
        </aside>
        <section class="right-panel">
            <SchedulesComponent section_error personal_error action=action read_signal=action.value() step/>
        </section>
        <Notifications modal set_modal/>
        <button on:click=move |_| {set_hide(false)} id="go-back"><CaretDoubleRight weight=IconWeight::Regular size="3vh"/></button>
    }
}
