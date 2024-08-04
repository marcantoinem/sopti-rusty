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

    let (is_active, set_active) = create_signal(false);

    view! {

        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/aep-schedule-website.css"/>

        // sets the document title
        <Title text="Générateur d'horaire"/>

        // content for this welcome page
        <Router>
            <header>
                <nav class=("active", is_active) class="flex-wrap overflow-x-hidden">
                    <span class="text-2xl font-semibold leading-none font-sans tracking-tight">"Générateur d'horaire de l'AEP"
                        <span class="text-amber-600">"v2"</span>
                    </span>
                    <span class="bg-red-200 text-red-800 text-lg font-sans tracking-tight font-medium me-2 px-2.5 py-0.5 rounded-full shrink">"Beta - "<a class="text-gray-800" href="https://horaires.aep.polymtl.ca/">"Retourner à l'ancien générateur"</a></span>
                    <A class="rounded-md font-medium text-gray-700 text-lg font-sans tracking-tight" href="/">"Générateur d'horaire"</A>
                    <A class="rounded-md font-medium text-gray-700 text-lg font-sans tracking-tight" href="/local">"Horaire d'un local"</A>
                    <A class="rounded-md font-medium text-gray-700 text-lg font-sans tracking-tight" href="/apropos">"À propos"</A>


                    <a href="https://git.step.polymtl.ca/Lemark/aep-schedule-generator-rusty/-/issues/new" class="sources pad-left"  target="_blank">
                        <span class="rounded-md font-medium text-gray-700 text-lg font-sans tracking-tight">"Signaler un bug"</span>
                        <Bug weight=IconWeight::Regular size="3vh"/>
                    </a>
                    <a href="https://git.step.polymtl.ca/Lemark/aep-schedule-generator-rusty" class="sources" target="_blank" ><span class="rounded-md font-medium text-gray-700 text-lg font-sans tracking-tight">"Sources "</span><GitlabLogo weight=IconWeight::Regular size="3vh"/></a>
                </nav>
                <div class=("active", is_active) class="hamburger" on:pointerdown=move |_| {
                    set_active.update(|active| {
                        *active = !*active;
                    });
                }>
                    <span class="hamburger-bar"></span>
                    <span class="hamburger-bar"></span>
                    <span class="hamburger-bar"></span>
                </div>
            </header>
            <main class="h-full">
                <Routes>
                    <Route path="/" view=GeneratorPage/>
                    <Route path="/apropos" view=HomePage/>
                    <Route path="/local" view=ClassRoomComponent/>
                </Routes>
            </main>
        </Router>
    }
}
