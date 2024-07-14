use crate::frontend::components::icons::{caret_double_right::CaretDoubleRight, IconWeight};
use crate::frontend::components::notifications::Notifications;
use crate::frontend::components::{options::form::OptionsForms, schedules::SchedulesComponent};
use aep_schedule_generator::algorithm::generation::SchedulesOptions;
use leptos::*;

#[component]
pub fn GeneratorPage() -> impl IntoView {
    let (hide, set_hide) = create_signal(false);
    // Creates a reactive value to update the button
    let action = create_action(move |s: &SchedulesOptions| {
        let s = s.clone();
        set_hide(true);
        async move { s.get_schedules().into_sorted_vec() }
    });

    let (modal, set_modal) = create_signal(None);

    view! {
        <aside class="left-panel" class=("hide-left-panel", hide)>
            <OptionsForms action=action/>
        </aside>
        <section class="right-panel">
            <SchedulesComponent read_signal=action.value()/>
        </section>
        <Notifications modal set_modal/>
        <button on:click=move |_| {set_hide(false)} id="go-back"><CaretDoubleRight weight=IconWeight::Regular size="3vh"/></button>
    }
}
