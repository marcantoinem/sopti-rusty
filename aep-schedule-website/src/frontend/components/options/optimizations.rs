use super::state::OptionState;
use leptos::*;

#[component]
pub fn SelectOptimizations(state: OptionState) -> impl IntoView {
    let (day_off, set_day_off) = state.day_off;
    let (morning, set_morning) = state.morning;
    let (finish_early, set_finish_early) = state.finish_early;

    view! {
        <input type="range" min="0" max="10" class="opti-slider" prop:value=day_off  on:change=move |ev| {
            set_day_off(event_target_value(&ev).parse::<u8>().unwrap())
        }/>
        <input type="range" min="-5" max="5" class="opti-slider" prop:value=morning  on:change=move |ev| {
            set_morning(event_target_value(&ev).parse::<i8>().unwrap())
        }/>
        <input type="range" min="0" max="10" class="opti-slider" prop:value=finish_early  on:change=move |ev| {
            set_finish_early(event_target_value(&ev).parse::<u8>().unwrap())
        }/>
    }
}
