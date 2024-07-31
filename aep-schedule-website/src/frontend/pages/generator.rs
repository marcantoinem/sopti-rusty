use crate::frontend::components::icons::{caret_double_right::CaretDoubleRight, IconWeight};
use crate::frontend::components::notifications::Notifications;
use crate::frontend::components::options::state::OptionState;
use crate::frontend::components::{options::form::OptionsForms, schedules::SchedulesComponent};
use aep_schedule_generator::algorithm::generation::SchedulesOptions;
use aep_schedule_generator::data::group_sigle::SigleGroup;
use leptos::*;

#[derive(Clone, Copy)]
pub struct SetModal(WriteSignal<Option<SigleGroup>>);

impl SetModal {
    pub fn from_context() -> WriteSignal<Option<SigleGroup>> {
        use_context::<Self>().unwrap().0
    }
}

#[derive(Clone, Copy)]
pub struct FirstGenerationDone(pub RwSignal<bool>);

#[component]
pub fn GeneratorPage() -> impl IntoView {
    let (hide, set_hide) = create_signal(false);
    let first_generation_done = create_rw_signal(false);

    // Creates a reactive value to update the button
    let action = create_action(move |s: &SchedulesOptions| {
        let mut s = s.clone();
        set_hide(true);
        s.apply_personal_schedule();
        async move { s.get_schedules().into_sorted_vec() }
    });

    let (modal, set_modal) = create_signal(None);

    let state = OptionState::default();

    provide_context(state);
    provide_context(SetModal(set_modal));
    provide_context(FirstGenerationDone(first_generation_done));

    view! {
        <aside class="left-panel" class=("hide-left-panel", hide)>
            <OptionsForms action/>
        </aside>
        <section class="right-panel">
            <SchedulesComponent action=action read_signal=action.value()/>
        </section>
        <Notifications modal set_modal/>
        <button on:click=move |_| {set_hide(false)} id="go-back"><CaretDoubleRight weight=IconWeight::Regular size="3vh"/></button>
    }
}
