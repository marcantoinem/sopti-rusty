use crate::frontend::components::icons::bug::Bug;
use crate::frontend::components::icons::gitlab_logo::GitlabLogo;
use crate::frontend::components::icons::IconWeight;
use crate::frontend::pages::apropos::HomePage;
use crate::frontend::pages::classroom::ClassRoomComponent;
use crate::frontend::pages::generator::GeneratorPage;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let (is_active, set_active) = create_signal(String::new());

    view! {

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/aep-schedule-website.css"/>

        // sets the document title
        <Title text="Générateur d'horaire"/>

        // content for this welcome page
        <Router>
            <header>
                <nav class=is_active>
                    <A href="/">"Générateur d'horaire"</A>
                    <A href="/local">"Horaire d'un local"</A>
                    <A href="/apropos">"À propos"</A>
                    <a href="https://git.step.polymtl.ca/Lemark/aep-schedule-generator-rusty/-/issues/new" class="sources pad-left"  target="_blank">
                        <span>"Signaler un bug"</span>
                        <Bug weight=IconWeight::Regular size="3vh"/>
                    </a>
                    <a href="https://git.step.polymtl.ca/Lemark/aep-schedule-generator-rusty" class="sources"  target="_blank" ><span>"Sources "</span><GitlabLogo weight=IconWeight::Regular size="3vh"/></a>
                </nav>
                <div class=move || is_active.get() + " hamburger" on:click=move |_| {
                    set_active.update(|text| {
                        if text == "active" {
                            text.clear();
                        } else {
                            text.push_str("active");
                        }
                    });
                }>
                    <span class="hamburger-bar"></span>
                    <span class="hamburger-bar"></span>
                    <span class="hamburger-bar"></span>
                </div>
            </header>
            <main>
                <Routes>
                    <Route path="/" view=GeneratorPage/>
                    <Route path="/apropos" view=HomePage/>
                    <Route path="/local" view=ClassRoomComponent/>
                </Routes>
            </main>
        </Router>
    }
}
