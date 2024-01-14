cfg_if::cfg_if!(if #[cfg(feature = "ssr")] {
    use axum::{
        routing::get,
        Router,
    };
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos::{get_configuration};
    use aep_schedule_website::fileserv::file_and_error_handler;
    use aep_schedule_website::frontend::app::App;
    use aep_schedule_website::backend::state::{AppState, server_fn_handler, leptos_routes_handler};

    #[tokio::main]
    async fn main() {
        simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");
        // Setting get_configuration(None) means we'll be using cargo-leptos's env values
        // For deployment these variables are:
        // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
        // Alternately a file can be specified such as Some("Cargo.toml")
        // The file would need to be included with the executable when moved to deployment
        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;
        let routes = generate_route_list(App);
        let state = AppState::new(leptos_options.clone(), routes.clone()).await;
        let addr = leptos_options.site_addr;
        // build our application with a route
        let app = Router::new()
            .route(
                "/api/*fn_name",
                get(server_fn_handler).post(server_fn_handler),
            )
            .leptos_routes_with_handler(routes, get(leptos_routes_handler))
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
});

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
