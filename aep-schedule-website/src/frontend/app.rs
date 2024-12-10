use crate::frontend::components::icons::bug::Bug;
use crate::frontend::components::icons::gitlab_logo::GitlabLogo;
use crate::frontend::components::icons::IconWeight;
use crate::frontend::pages::apropos::HomePage;
use crate::frontend::pages::classroom::ClassRoomComponent;
use crate::frontend::pages::generator::GeneratorPage;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{FlatRoutes, Route, Router, A},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="fr">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <link rel="stylesheet" id="leptos" href="/pkg/aep-schedule-website.css"/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn Nav() -> impl IntoView {
    let (is_active, set_active) = signal(false);

    view! {
        <header>
            <nav class=("active", is_active) class="flex-wrap overflow-x-hidden">
                <span class="text-2xl font-semibold leading-none font-sans tracking-tight">"Générateur d'horaire de l'AEP"
                    <span class="text-amber-600">"v2"</span>
                </span>
                <span class="bg-red-200 text-red-800 text-lg font-sans tracking-tight font-medium me-2 px-2.5 py-0.5 rounded-full shrink">"Beta - "<a class="text-gray-800" href="https://horaires.aep.polymtl.ca/">"Retourner à l'ancien générateur"</a></span>
                <A href="/"><span class="rounded-md font-medium text-gray-700 text-lg font-sans tracking-tight">"Générateur d'horaire"</span></A>
                <A href="/local"><span class="rounded-md font-medium text-gray-700 text-lg font-sans tracking-tight">"Horaire d'un local"</span></A>
                <A href="/apropos"><span class="rounded-md font-medium text-gray-700 text-lg font-sans tracking-tight">"À propos"</span></A>


                <a href="https://forms.gle/u5AWgGx7vcLbCPCc7" class="sources pad-left"  target="_blank">
                    <span class="rounded-md font-medium text-gray-700 text-lg font-sans tracking-tight">"Signaler un bug"</span>
                    <Bug size="3vh"/>
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
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Title text="Générateur d'horaire"/>
            <Nav/>
            <main class="h-full">
                <FlatRoutes fallback=|| "Not found">
                    <Route path=StaticSegment("/") view=GeneratorPage/>
                    <Route path=StaticSegment("/apropos") view=HomePage/>
                    <Route path=StaticSegment("/local") view=ClassRoomComponent/>
                </FlatRoutes>
            </main>
        </Router>
    }
}
