use leptos::*;

#[component]
pub fn Tab(active_tab: ReadSignal<String>, tab_id: String, children: Children) -> impl IntoView {
    view! {
        <div class="relative card tab shrink w-full overflow-y-auto" class=("hidden", {move || tab_id != active_tab.get()})>
            {children()}
        </div>
    }
}
