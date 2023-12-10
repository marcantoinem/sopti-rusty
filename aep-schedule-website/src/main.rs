#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use aep_schedule_website::backend::state::AppState;
    use aep_schedule_website::fileserv::file_and_error_handler;
    use aep_schedule_website::frontend::app::App;
    use axum::{routing::post, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let state = AppState::new(leptos_options.clone());
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&state, routes, App)
        .fallback(file_and_error_handler)
        .with_state(state.clone());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    let _ = tokio::join!(
        axum::Server::bind(&addr).serve(app.into_make_service()),
        state.update_courses()
    );
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
