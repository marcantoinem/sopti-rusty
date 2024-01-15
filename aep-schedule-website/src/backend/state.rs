cfg_if::cfg_if! { if #[cfg(feature = "ssr")] {
    use axum::extract::FromRef;
    use leptos::*;
    use leptos_router::RouteListing;
    use tokio::sync::RwLock;
    use std::io::BufReader;
    use std::time::Duration;
    use std::sync::Arc;
    use std::fs::File;
    use std::fs;
    use aep_schedule_generator::data::courses::Courses;
    use aep_schedule_generator::data::time::calendar::Calendar;
    use axum::{
        response::{IntoResponse},
        extract::{Path, State, RawQuery},
        http::{Request, header::HeaderMap},
        body::Body as AxumBody,
    };
    use leptos_axum::{handle_server_fns_with_context};
    use leptos::{provide_context};
    use crate::frontend::app::App;
    use axum::response::Response;

    /// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
    /// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
    #[derive(FromRef, Debug, Clone)]
    pub struct AppState {
        pub leptos_options: LeptosOptions,
        pub courses: Arc<RwLock<Courses>>,
        pub calendar: Arc<RwLock<Calendar>>,
        pub routes: Vec<RouteListing>,
    }

    impl AppState {
        pub async fn new(leptos_options: LeptosOptions, routes: Vec<RouteListing>) -> Self {
            let client = reqwest::Client::builder()
                .user_agent("NCSA Mosaic/1.0 (X11;SunOS 4.1.4 sun4m)")
                .build()
                .unwrap();

            let horsage = client.get("https://cours.polymtl.ca/Horaire/public/horsage.csv").send().await.unwrap();
            let horsage = horsage.text().await.unwrap();


            let fermes = client.get("https://cours.polymtl.ca/Horaire/public/fermes.csv").send().await.unwrap();
            let fermes = fermes.text().await.unwrap();

            fs::write("horsage.csv", horsage).expect("Unable to write file");
            fs::write("fermes.csv", fermes).expect("Unable to write file");
            let horsage = BufReader::new(File::open("horsage.csv").unwrap());
            let fermes = BufReader::new(File::open("fermes.csv").unwrap());
            let courses = Arc::new(RwLock::new(Courses::from_csv(horsage, fermes)));

            let alternance = BufReader::new(File::open("alternance.csv").unwrap());
            let calendar = Arc::new(RwLock::new(Calendar::from_csv(alternance)));

            Self {
                routes,
                leptos_options,
                calendar,
                courses,
            }
        }
        pub async fn update_courses(&self) {
            // The reverse proxy at Poly needs an user agent. So don't forget to put the most cringe user agent.
            let client = reqwest::Client::builder()
                .user_agent("NCSA Mosaic/1.0 (X11;SunOS 4.1.4 sun4m)")
                .build()
                .unwrap();

            loop {
                tokio::time::sleep(Duration::from_secs(5*60)).await;
                let Ok(horsage) = client.get("https://cours.polymtl.ca/Horaire/public/horsage.csv").send().await
                    else {continue};
                let Ok(horsage) = horsage.text().await else {continue};


                let Ok(fermes) = client.get("https://cours.polymtl.ca/Horaire/public/fermes.csv").send().await
                    else {continue};
                let Ok(fermes) = fermes.text().await else {continue};

                fs::write("horsage.csv", horsage).expect("Unable to write file");
                fs::write("fermes.csv", fermes).expect("Unable to write file");

                let horsage = BufReader::new(File::open("horsage.csv").unwrap());
                let fermes = BufReader::new(File::open("fermes.csv").unwrap());
                let courses = Courses::from_csv(horsage, fermes);
                *self.courses.write().await = courses;
            }
        }
        pub async fn courses() -> Result<Arc<RwLock<Courses>>, ServerFnError> {
            Ok(use_context::<Arc<RwLock<Courses>>>().ok_or(ServerFnError::ServerError("Courses not available".to_string()))?)
        }
        pub async fn calendar() -> Result<Arc<RwLock<Calendar>>, ServerFnError> {
            Ok(use_context::<Arc<RwLock<Calendar>>>().ok_or(ServerFnError::ServerError("Courses not available".to_string()))?)
        }
    }

    pub async fn server_fn_handler(State(app_state): State<AppState>, path: Path<String>, headers: HeaderMap, raw_query: RawQuery, request: Request<AxumBody>) -> impl IntoResponse {
        handle_server_fns_with_context(path, headers, raw_query, move || {
            provide_context(app_state.courses.clone());
        }, request).await
    }

    pub async fn leptos_routes_handler(State(app_state): State<AppState>, req: Request<AxumBody>) -> Response {
        let handler = leptos_axum::render_route_with_context(app_state.leptos_options.clone(),
            app_state.routes.clone(),
            move || {
                provide_context(app_state.courses.clone());
            },
            App
        );
        handler(req).await.into_response()
    }
}}
