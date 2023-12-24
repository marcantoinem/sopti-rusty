use leptos::*;

#[component]
pub fn NumberInput(#[prop(optional, into)] value: RwSignal<u8>, max: u8) -> impl IntoView {
    let on_input = move |ev| {
        value.set(event_target_value(&ev).parse::<u8>().unwrap());
    };

    view! {
        <input
            on:input=on_input
            type="number"
            min="0"
            max=max
            prop:value=value
            class="number-input"
        />
    }
}
