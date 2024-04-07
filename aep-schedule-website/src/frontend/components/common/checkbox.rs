use leptos::*;

#[component]
pub fn CheckboxChip(
    #[prop(optional, into)] value: RwSignal<bool>,
    #[prop(optional, into)] class: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div on:click=move |_| {value.update(|b| *b = !*b)} class="chips ".to_owned() + class>
            <input
                type="checkbox"
                prop:checked=value
            />
            {children()}
        </div>
    }
}
