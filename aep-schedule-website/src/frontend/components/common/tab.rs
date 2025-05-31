use leptos::prelude::*;

#[component]
pub fn Tab(active_tab: ReadSignal<String>, tab_id: String, children: Children) -> impl IntoView {
    view! {
        <div
            class="relative bg-gray-100 shadow-md text-gray-900 text-center rounded-md p-0 font-bold tab shrink w-full overflow-y-auto"
            class=("hidden", { move || tab_id != active_tab.get() })
        >
            {children()}
        </div>
    }
}
