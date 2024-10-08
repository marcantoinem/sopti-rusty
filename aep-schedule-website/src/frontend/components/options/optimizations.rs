use crate::frontend::{
    components::icons::{
        calendar_check::CalendarCheck, house::House, sun::Sun, sun_horizon::SunHorizon, IconWeight,
    },
    state::OptionState,
};
use leptos::*;
use std::cmp;

fn weight(input: u8) -> IconWeight {
    match input {
        0 => IconWeight::Thin,
        1 => IconWeight::Light,
        2 => IconWeight::Regular,
        3 => IconWeight::Bold,
        _ => IconWeight::Fill,
    }
}

#[component]
pub fn SelectOptimizations<F>(state: OptionState, submit: F) -> impl IntoView
where
    F: Fn() + Copy + 'static,
{
    let weight_house = create_memo(move |_| weight(state.day_off.get()));
    let weight_early = create_memo(move |_| weight((-cmp::min(0, state.morning.get())) as u8));
    let weight_morning = create_memo(move |_| weight(cmp::max(0, state.morning.get()) as u8));
    let weight_finish = create_memo(move |_| weight(state.finish_early.get()));

    view! {
        <div class="three-col">
            <div class="flex flex-col items-center">
                <House weight=weight_house size="10vh"/>
                <p class="font-sans font-medium tracking-tight">"Plus de congés"</p>
                <input type="range" min="0" max="4" class="lg:w-24 w-16 accent-amber-500" prop:value=state.day_off  on:input=move |ev| {
                    state.day_off.set(event_target_value(&ev).parse::<u8>().unwrap());
                    submit();
                }/>
            </div>
            <div class="flex flex-col items-center">
                <div class="flex">
                    <SunHorizon weight=weight_early size="10vh"/>
                    <Sun weight=weight_morning size="10vh"/>
                </div>
                <p class="font-sans font-medium tracking-tight">"Cours plus tôt ou plus tard"</p>
                <input type="range" min="-4" max="4" class="lg:w-48 w-32 accent-amber-500" prop:value=state.morning  on:input=move |ev| {
                    state.morning.set(event_target_value(&ev).parse::<i8>().unwrap());
                    submit();
                }/>
            </div>
            <div class="flex flex-col items-center">
                <CalendarCheck weight=weight_finish size="10vh"/>
                <p class="font-sans font-medium tracking-tight">"Finir plus tôt"</p>
                <input type="range" min="0" max="4" class="lg:w-24 w-16 accent-amber-500" prop:value=state.finish_early  on:input=move |ev| {
                    state.finish_early.set(event_target_value(&ev).parse::<u8>().unwrap());
                    submit();
                }
                />
            </div>
        </div>
    }
}
