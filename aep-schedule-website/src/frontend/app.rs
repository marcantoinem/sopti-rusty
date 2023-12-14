use crate::frontend::pages::generator::GeneratorPage;
use crate::frontend::pages::home::HomePage;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/aep-schedule-website.css"/>

        // sets the document title
        <Title text="Générateur d'horaire de l'AEP"/>

        // content for this welcome page
        <Router>
            <nav>
                <a href="/">"Accueil"</a>
                <a href="/generateur">"Générateur d'horaire"</a>
            </nav>
            <main>
                <Routes>
                    <Route path="/" view=HomePage/>
                    <Route path="/generateur" view=GeneratorPage/>
                </Routes>
            </main>
        </Router>
    }
}
