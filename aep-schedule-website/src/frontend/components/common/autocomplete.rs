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

fn get_suggestions<F: FnMut(String) -> () + 'static>(
    sorted_possibilities: Vec<AutoCompleteOption>,
    input: RwSignal<String>,
    input_value: String,
    mut submit: F,
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
        submit(input_value);
        input.set("".to_string());
        suggestion_range.set(0..0);
        return;
    }
    suggestion_range.set(bottom..top);
}

#[component]
pub fn AutoComplete<F: FnMut(String) -> () + Copy + Clone + 'static>(
    suggestion_list: Vec<AutoCompleteOption>,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] class: String,
    id: &'static str,
    mut submit: F,
) -> impl IntoView {
    let input = create_rw_signal(String::new());
    let (suggestion_range, set_suggestion_range) = create_signal(0..0);
    let suggestions = suggestion_list.clone();

    let on_input = move |ev| {
        let value = event_target_value(&ev);
        input.set(value.clone());
        let suggestion_list = suggestion_list.clone();
        get_suggestions(suggestion_list, input, value, submit, set_suggestion_range);
    };

    view! {
        <div class="search-container" class=class>
            <input type="text" class="search-bar" on:input=on_input placeholder=placeholder prop:value=input id=id/>
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
