use crate::frontend::components::icons::plus_circle::PlusCircle;
use leptos::*;
use std::{cmp, ops::Range};

#[derive(Clone, PartialEq)]
pub struct AutoCompleteOption {
    value: String,
    label: String,
}

impl AutoCompleteOption {
    pub fn new(value: String, label: String) -> Self {
        Self { value, label }
    }
}

fn get_suggestions(
    sorted_possibilities: Vec<AutoCompleteOption>,
    is_hidden: RwSignal<bool>,
    input_value: String,
    suggestion_range: WriteSignal<Range<usize>>,
) {
    let input_value = input_value.trim();
    let input_value = input_value.to_uppercase();
    let i = input_value.len();
    let bottom = sorted_possibilities
        .partition_point(|c| c.value[0..cmp::min(i, c.value.len())] < input_value[0..i]);
    let top = sorted_possibilities
        .partition_point(|c| c.value[0..cmp::min(i, c.value.len())] <= input_value[0..i]);
    if bottom < sorted_possibilities.len() && sorted_possibilities[bottom].value == input_value {
        is_hidden.set(false);
        suggestion_range.set(bottom..top);
        return;
    }

    is_hidden.set(true);

    suggestion_range.set(bottom..top);
}

#[component]
pub fn AutoComplete<F: FnMut(String) + Copy + Clone + 'static>(
    suggestion_list: Vec<AutoCompleteOption>,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] class: String,
    id: &'static str,
    mut submit: F,
) -> impl IntoView {
    let input = create_rw_signal(String::new());
    let (suggestion_range, set_suggestion_range) = create_signal(0..0);
    let suggestions = suggestion_list.clone();
    let is_hidden = create_rw_signal(true);

    let on_input = move |ev| {
        let value = event_target_value(&ev);
        input.set(value.clone());
        let suggestion_list = suggestion_list.clone();
        get_suggestions(suggestion_list, is_hidden, value, set_suggestion_range);
    };

    let button_theme = move || {
        let is_hidden = is_hidden.get();
        match is_hidden {
            true => "absolute text-black top-0.5 right-0.5 hidden",
            false => "absolute text-black top-0.5 right-0.5",
        }
    };

    view! {
        <div class="relative search-container ".to_owned() + &class>
            <input type="text" class="py-2 px-3 block w-full border-gray-200 rounded-lg text-sm focus:border-blue-500 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none text-black" on:input=on_input placeholder=placeholder prop:value=input id=id on:keyup=move |ev| {
                if ev.key() == "Enter" && !is_hidden.get() {
                    let input = input.get().trim().to_uppercase();
                    submit(input);
                }
            }
            />
            <button class=button_theme on:click=move |_| {
                let input = input.get().trim().to_uppercase();
                submit(input);
            }>
                <PlusCircle size="2em"/>
            </button>
            <div class="result-box">
                {suggestions.into_iter().enumerate().map(|(i, autocomplete)| view!{
                    <div
                        class=("hidden", move || {!suggestion_range.get().contains(&i)})
                        on:click=move |_| submit(autocomplete.value.clone())
                    >
                        {autocomplete.label}
                    </div>
                }).collect_view()}
            </div>
        </div>
    }
}
