use crate::components::icons::caret_double_right::CaretDoubleRight;
use crate::components::{options::form::OptionsForms, schedules::SchedulesComponent};
use crate::state::action_add_course::ActionAddCourse;
use crate::state::OptionState;
use leptos::prelude::*;

#[component]
pub fn GeneratorPage() -> impl IntoView {
    let state = OptionState::default();
    let hide = state.hide;
    provide_context(state);
    provide_context(ActionAddCourse::new(state));

    view! {
        <aside class="left-panel" class=("hide-left-panel", hide)>
            <OptionsForms />
        </aside>
        <div
            class="right-panel"
            on:scroll:target=move |ev| {
                use web_sys::wasm_bindgen::JsCast;
                let target = ev.target().dyn_into::<web_sys::Element>().unwrap();
                let scroll_top = target.scroll_top() as f64;
                let client_height = target.client_height() as f64;
                let scroll_height = target.scroll_height() as f64;
                if (scroll_top + client_height >= scroll_height - 500.0) && state.step.get() == 6 {
                    state.regenerate();
                }
            }
        >
            <SchedulesComponent />
        </div>
        <button on:pointerdown=move |_| { hide.set(false) } id="go-back">
            <CaretDoubleRight size="3vh" />
        </button>
    }
}
