use super::state::OptionState;
use leptos::*;

#[component]
pub fn SelectOptimizations(state: OptionState) -> impl IntoView {
    view! {
        <input type="range" min="0" max="10" class="opti-slider input-item" prop:value=state.day_off  on:change=move |ev| {
            state.day_off.set(event_target_value(&ev).parse::<u8>().unwrap())
        }/>
        <input type="range" min="-5" max="5" class="opti-slider input-item" prop:value=state.morning  on:change=move |ev| {
            state.morning.set(event_target_value(&ev).parse::<i8>().unwrap())
        }/>
        <input type="range" min="0" max="10" class="opti-slider input-item" prop:value=state.finish_early  on:change=move |ev| {
            state.finish_early.set(event_target_value(&ev).parse::<u8>().unwrap())
        }/>
    }
}
