use crate::frontend::components::icons::{caret_double_right::CaretDoubleRight, IconWeight};
use crate::frontend::components::notifications::Notifications;
use crate::frontend::components::{options::form::OptionsForms, schedules::SchedulesComponent};
use crate::frontend::state::OptionState;
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
    let (modal, set_modal) = create_signal(None);
    let state = OptionState::default();

    provide_context(state);
    provide_context(SetModal(set_modal));
    provide_context(FirstGenerationDone(first_generation_done));

    view! {
        <aside class="left-panel" class=("hide-left-panel", hide)>
            <OptionsForms/>
        </aside>
        <section class="right-panel">
            <SchedulesComponent/>
        </section>
        <Notifications modal set_modal/>
        <button on:click=move |_| {set_hide(false)} id="go-back"><CaretDoubleRight weight=IconWeight::Regular size="3vh"/></button>
    }
}
