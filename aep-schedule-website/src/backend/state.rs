use crate::frontend::app::App;
use aep_schedule_generator::data::courses::Courses;
use aep_schedule_generator::data::time::calendar::Calendar;
use axum::extract::FromRef;
use axum::response::Response;
use axum::{
    body::Body as AxumBody,
    extract::{Path, RawQuery, State},
    http::{header::HeaderMap, Request},
};
use leptos::*;
use leptos_router::RouteListing;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

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
        let horsage = client
            .get("https://cours.polymtl.ca/Horaire/public/horsage.csv")
            .send()
            .await
            .unwrap();
        let horsage = horsage.text().await.unwrap();
        let fermes = client
            .get("https://cours.polymtl.ca/Horaire/public/fermes.csv")
            .send()
            .await
            .unwrap();
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
            tokio::time::sleep(Duration::from_secs(5 * 60)).await;
            let Ok(horsage) = client
                .get("https://cours.polymtl.ca/Horaire/public/horsage.csv")
                .send()
                .await
            else {
                continue;
            };
            let Ok(horsage) = horsage.text().await else {
                continue;
            };
            let Ok(fermes) = client
                .get("https://cours.polymtl.ca/Horaire/public/fermes.csv")
                .send()
                .await
            else {
                continue;
            };
            let Ok(fermes) = fermes.text().await else {
                continue;
            };
            fs::write("horsage.csv", horsage).expect("Unable to write file");
            fs::write("fermes.csv", fermes).expect("Unable to write file");
            let horsage = BufReader::new(File::open("horsage.csv").unwrap());
            let fermes = BufReader::new(File::open("fermes.csv").unwrap());
            let courses = Courses::from_csv(horsage, fermes);
            *self.courses.write().await = courses;
        }
    }
}
