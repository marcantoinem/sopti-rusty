use crate::frontend::app::App;
use aep_schedule_generator::data::courses::Courses;
use aep_schedule_generator::data::time::calendar::Calendar;
use axum::{
    body::Body as AxumBody,
    extract::FromRef,
    extract::State,
    http::Request,
    response::{IntoResponse, Response},
};
use leptos::*;
use leptos_axum::handle_server_fns_with_context;
use leptos_router::RouteListing;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::time::Duration;
use std::{fs, sync::Mutex};
use tokio::sync::RwLock;

use super::notification::users::UsersToNotify;

const HORSAGE_URL: &str = "https://cours.polymtl.ca/Horaire/public/horsage.csv";
const FERME_URL: &str = "https://cours.polymtl.ca/Horaire/public/fermes.csv";

/// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
/// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
#[derive(FromRef, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub courses: Arc<RwLock<Courses>>,
    pub calendar: Arc<RwLock<Calendar>>,
    pub users_to_notify: Arc<Mutex<UsersToNotify>>,
    pub routes: Vec<RouteListing>,
}

impl AppState {
    pub async fn new(leptos_options: LeptosOptions, routes: Vec<RouteListing>) -> Self {
        let client = reqwest::Client::builder()
            .user_agent("NCSA Mosaic/1.0 (X11;SunOS 4.1.4 sun4m)")
            .build()
            .unwrap();
        let horsage = client.get(HORSAGE_URL).send().await.unwrap();
        let horsage = horsage.text().await.unwrap();
        let fermes = client.get(FERME_URL).send().await.unwrap();
        let fermes = fermes.text().await.unwrap();
        fs::write("horsage.csv", horsage).expect("Unable to write file");
        fs::write("fermes.csv", fermes).expect("Unable to write file");
        let horsage = BufReader::new(File::open("horsage.csv").unwrap());
        let fermes = BufReader::new(File::open("fermes.csv").unwrap());
        let courses = Arc::new(RwLock::new(Courses::from_csv(horsage, fermes)));
        let alternance = BufReader::new(File::open("alternance.csv").unwrap());
        let calendar = Arc::new(RwLock::new(Calendar::from_csv(alternance)));
        let users_to_notify = Arc::new(Mutex::new(UsersToNotify::new()));

        Self {
            routes,
            leptos_options,
            calendar,
            users_to_notify,
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
            tokio::time::sleep(Duration::from_secs(15 * 60)).await;
            let Ok(horsage) = client.get(HORSAGE_URL).send().await else {
                continue;
            };
            let Ok(horsage) = horsage.text().await else {
                continue;
            };
            let Ok(fermes) = client.get(FERME_URL).send().await else {
                continue;
            };
            let Ok(fermes) = fermes.text().await else {
                continue;
            };
            fs::write("horsage.csv", horsage).expect("Unable to write file");
            fs::write("fermes.csv", fermes).expect("Unable to write file");
            let horsage = BufReader::new(File::open("horsage.csv").unwrap());
            let fermes = BufReader::new(File::open("fermes.csv").unwrap());
            let opened_course = self.courses.write().await.update(horsage, fermes);
            self.users_to_notify
                .lock()
                .unwrap()
                .send_opened(opened_course)
                .await;
        }
    }

    pub async fn courses() -> Arc<RwLock<Courses>> {
        use_context::<Arc<RwLock<Courses>>>().unwrap()
    }

    pub async fn calendar() -> Arc<RwLock<Calendar>> {
        use_context::<Arc<RwLock<Calendar>>>().unwrap()
    }

    pub async fn users_to_notify() -> Arc<Mutex<UsersToNotify>> {
        use_context::<Arc<Mutex<UsersToNotify>>>().unwrap()
    }
}

pub async fn server_fn_handler(
    State(app_state): State<AppState>,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    handle_server_fns_with_context(
        move || {
            provide_context(app_state.calendar.clone());
            provide_context(app_state.courses.clone());
            provide_context(app_state.users_to_notify.clone());
        },
        request,
    )
    .await
}

pub async fn leptos_routes_handler(
    State(app_state): State<AppState>,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_route_with_context(
        app_state.leptos_options.clone(),
        app_state.routes.clone(),
        move || {
            provide_context(app_state.calendar.clone());
            provide_context(app_state.courses.clone());
            provide_context(app_state.users_to_notify.clone());
        },
        App,
    );
    handler(req).await.into_response()
}
