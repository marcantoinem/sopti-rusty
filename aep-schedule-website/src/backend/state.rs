cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::extract::FromRef;
        use leptos::*;
        use tokio::sync::RwLock;
        use std::io::BufReader;
        use std::time::Duration;
        use std::sync::Arc;
        use std::fs::File;
        use aep_schedule_generator::data::courses::Courses;

        /// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
        /// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
        #[derive(FromRef, Debug, Clone)]
        pub struct AppState {
            pub leptos_options: LeptosOptions,
            pub courses: Arc<RwLock<Courses>>,
        }

        impl AppState {
            pub fn new(leptos_options: LeptosOptions) -> Self {
                let horsage = BufReader::new(File::open("horsage.csv").unwrap());
                let fermes = BufReader::new(File::open("fermes.csv").unwrap());
                let courses = Arc::new(RwLock::new(Courses::from_csv(horsage, fermes)));
                Self {
                    leptos_options,
                    courses,
                }
            }
            pub async fn update_courses(&self) {
                loop {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    let horsage = BufReader::new(File::open("horsage.csv").unwrap());
                    let fermes = BufReader::new(File::open("fermes.csv").unwrap());
                    let courses = Courses::from_csv(horsage, fermes);
                    *self.courses.write().await = courses;
                }
            }
            pub async fn courses() -> Arc<RwLock<Courses>> {
                use_context::<Self>().unwrap().courses
            }
        }
    }
}
