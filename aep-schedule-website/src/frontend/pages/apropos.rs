use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="p-1 w-full h-full">
            <div class="container flex flex-col justify-center p-4 mx-auto md:p-8">
                <p class="p-2 text-sm font-medium tracking-wider text-center uppercase">"Comment ça marche?"</p>
                <h2 class="mb-12 text-4xl font-bold leading-none text-center sm:text-5xl">"Foires aux questions"</h2>
                <div class="grid gap-10 md:gap-8 sm:p-3 md:grid-cols-2 lg:px-12">
                    <div>
                        <h3 class="font-semibold">"Pourquoi avoir refait l'ancien générateur?"</h3>
                        <p class="mt-1 dark:text-gray-600">
                            "L'ancien générateur d'horaire était en mode maintenance depuis au moins 15 ans et changer l'architecture aurait été très difficile, en effet la vénérable base de code contenait littéralement 5 langages de programmation différents!"<br/> "- Marc-Antoine Manningham"
                        </p>
                    </div>
                    <div>
                        <h3 class="font-semibold">"Qui sont les auteurs de la refonte du générateur d'horaire?"</h3>
                        <p class="mt-1 dark:text-gray-600">"Marc-Antoine Manningham - Création du frontend et du backend entièrement en Rust. Raphael Salvas, Achille Saint-Hillier, Sunnee Chevalier et Gabriel Billard - Inspiration de leur TP3 de LOG2420 pour l'interface de la refonte du générateur"</p>
                    </div>
                    <div>
                        <h3 class="font-semibold">"Comment le générateur génère aussi vite?"</h3>
                        <p class="mt-1 dark:text-gray-600">"D'abord, les horaires sont généré directement dans le navigateur de l'utilisateur en WebAssembly plutôt que sur le serveur. Aussi, plutôt que de générer tout les horaires, juste le top des horaires sont générés. Plusieurs techniques pour couper l'espace de combinaisons sont utilisées pour éliminer les branches qui ne respectent pas les contraintes ou qui sont sous-optimales. (branch and bound)"</p>
                    </div>
                    <div>
                        <h3 class="font-semibold">"Si j'ai des suggestions où aller les mettre?"</h3>
                        <p class="mt-1 dark:text-gray-600">"Le bouton signaler un bug peut aussi être utilisé pour soumettre une fonctionnalité sur le GitLab. Sinon, les contributions sont aussi les bienvenues si vous êtes prêt à faire un peu de Rust!"</p>
                    </div>
                </div>
            </div>
        </section>
    }
}
