use crate::frontend::components::common::schedule::Schedule;
use leptos::*;

#[component]
pub fn PersonalTimeSelector() -> impl IntoView {
    view! {
        <Schedule col_height="0.4em"></Schedule>
    }
}
