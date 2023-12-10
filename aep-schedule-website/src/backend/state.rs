use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::extract::FromRef;
        use leptos::LeptosOptions;
        use tokio::sync::RwLock;
        use std::sync::Arc;
        use aep_schedule_generator::data::courses::Courses;

        /// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
        /// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
        #[derive(FromRef, Debug, Clone)]
        pub struct AppState {
            pub leptos_options: LeptosOptions,
            pub courses: Arc<RwLock<Courses>>,
        }
    }
}
