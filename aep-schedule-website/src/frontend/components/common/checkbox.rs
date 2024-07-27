use leptos::*;

#[component]
pub fn CheckboxChip<F>(
    #[prop(optional, into)] value: RwSignal<bool>,
    #[prop(optional, into)] class: &'static str,
    submit: F,
    children: Children,
) -> impl IntoView
where
    F: Fn() + Copy + 'static,
{
    view! {
        <div on:click=move |_| {
            value.update(|b| *b = !*b);
            submit();
        } class="chips ".to_owned() + class>
            <input
                type="checkbox"
                prop:checked=value
            />
            {children()}
        </div>
    }
}
