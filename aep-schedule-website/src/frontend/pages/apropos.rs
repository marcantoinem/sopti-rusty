use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="p-1 w-full h-full">
            <h2>"Auteurs"</h2>
            <ul>
                <li>"Marc-Antoine Manningham - Création du frontend et du backend entièrement en Rust"</li>
                <li>"Raphael Salvas, Achille Saint-Hillier, Sunnee Chevalier et Gabriel Billard - Inspiration de leur TP3 de LOG2420 pour le design du générateur"</li>
            </ul>
            <div class="container mx-auto flex flex-col items-center text-center lg:px-32 xl:max-w-1/2">
                <h1 class="text-5xl font-bold leading-none sm:text-6xl">"Générateur d'horaire de l'AEP"
                    <span class="text-amber-600">"v2"</span>
                </h1>
                <p class="px-8 mt-8 mb-12 text-lg">
                    "Vous en avez assez d'avoir des horaires horribles fait pas le registrariat? Vous êtes au bon endroit. Le générateur d'horaire v2 est une réécriture plus performante, intuive et moderne du générateur d'horaire. Il vous aidera à trouver un horaire parfait."
                </p>
                <div class="flex flex-col space-y-4 sm:items-center sm:justify-center sm:flex-row sm:space-y-0 sm:space-x-4 lg:justify-start">
                    <a rel="noopener noreferrer" href="#" class="px-8 py-3 text-lg font-semibold rounded dark:bg-amber-600 dark:text-gray-50">"Générer un horaire"</a>
                    <a rel="noopener noreferrer" href="#" class="px-8 py-3 text-lg font-semibold border rounded dark:border-gray-800">"Horaire d'un local"</a>
                </div>
            </div>
        </section>
    }
}
