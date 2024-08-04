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
    let first_generation_done = create_rw_signal(false);
    let (modal, set_modal) = create_signal(None);
    let state = OptionState::default();

    provide_context(state);
    provide_context(SetModal(set_modal));
    provide_context(FirstGenerationDone(first_generation_done));

    view! {
        <aside class="left-panel" class=("hide-left-panel", state.hide)>
            <OptionsForms/>
        </aside>
        <section class="right-panel" on:scroll=move |ev| {
            use web_sys::wasm_bindgen::JsCast;

            let target = ev
                .target()
                .unwrap()
                .dyn_into::<web_sys::Element>()
                .unwrap();
            let scroll_top = target.scroll_top() as f64;
            let client_height = target.client_height() as f64;
            let scroll_height = target.scroll_height() as f64;
            if (scroll_top + client_height >= scroll_height - 500.0) && state.step.get() == 6 {
                state.regenerate();
            }

        }>
            <SchedulesComponent/>
        </section>
        <Notifications modal set_modal/>
        <button on:pointerdown=move |_| {state.hide.set(false)} id="go-back"><CaretDoubleRight weight=IconWeight::Regular size="3vh"/></button>
    }
}
