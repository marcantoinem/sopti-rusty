use axum::extract::FromRef;
use generator::data::courses::Courses;
use generator::icalendar::calendar::Calendar;
use leptos::prelude::*;
use leptos_axum::AxumRouteListing;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

const HORSAGE_URL: &str = "https://cours.polymtl.ca/Horaire/public/horsage.csv";
const FERME_URL: &str = "https://cours.polymtl.ca/Horaire/public/fermes.csv";

/// This takes advantage of Axum's SubStates feature by deriving FromRef. This is the only way to have more than one
/// item in Axum's State. Leptos requires you to have leptosOptions in your State struct for the leptos route handlers
#[derive(FromRef, Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub courses: Arc<RwLock<Courses>>,
    pub calendar: Arc<RwLock<Calendar>>,
    pub routes: Vec<AxumRouteListing>,
}

impl AppState {
    pub async fn new(leptos_options: LeptosOptions, routes: Vec<AxumRouteListing>) -> Self {
        #[cfg(not(debug_assertions))]
        {
            // Don't spam Poly when reloading the website in debug mode
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
        }
        let horsage = BufReader::new(File::open("horsage.csv").unwrap());
        let fermes = BufReader::new(File::open("fermes.csv").unwrap());
        let courses = Arc::new(RwLock::new(Courses::from_csv(horsage, fermes)));
        let alternance = BufReader::new(File::open("alternance.csv").unwrap());
        let calendar = Arc::new(RwLock::new(Calendar::from_csv(alternance)));

        Self {
            routes,
            leptos_options,
            calendar,
            //users_to_notify,
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
            let _opened_course = self.courses.write().await.update(horsage, fermes);
        }
    }

    pub async fn courses() -> Arc<RwLock<Courses>> {
        use_context::<Arc<RwLock<Courses>>>().unwrap()
    }

    pub async fn calendar() -> Arc<RwLock<Calendar>> {
        use_context::<Arc<RwLock<Calendar>>>().unwrap()
    }
}
