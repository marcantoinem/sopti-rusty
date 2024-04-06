use crate::frontend::pages::generator::GeneratorPage;
use crate::frontend::pages::home::HomePage;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use phosphor_leptos::{Bug, GitlabLogo, IconWeight};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/aep-schedule-website.css"/>

        // sets the document title
        <Title text="Générateur d'horaire"/>

        // content for this welcome page
        <Router>
            <nav>
                <A href="/">"Accueil"</A>
                <A href="/generateur">"Générateur d'horaire"</A>
                <a href="https://git.step.polymtl.ca/Lemark/aep-schedule-generator-rusty/-/issues/new" class="sources pad-left"  target="_blank">
                    <span>"Reporter un bug"</span>
                    <Bug weight=IconWeight::Regular size="3vh"/>
                </a>
                <a href="https://git.step.polymtl.ca/Lemark/aep-schedule-generator-rusty" class="sources"  target="_blank" ><span>"Sources "</span><GitlabLogo weight=IconWeight::Regular size="3vh"/></a>
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
