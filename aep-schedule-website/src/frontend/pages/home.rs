use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section class="home">
            <h1>"Générateur d'horaire de l'AEP v2"</h1>
            <p>
                "Vous en avez assez d'avoir des horaires horribles fait pas le registrariat? Vous êtes au bon endroit. Le générateur d'horaire v2 est une réécriture plus performante, intuive et moderne du générateur d'horaire. Il vous aidera à trouver un horaire parfait."
            </p>
            <h2>"Auteurs"</h2>
            <ul>
                <li>"Marc-Antoine Manningham - Création de la frontend et du backend entièrement en Rust"</li>
                <li>"Raphael Salvas, Achille Saint-Hillier, Sunnee Chevalier et Gabriel Billard - Inspiration de leur TP3 de LOG2420 pour le design du générateur"</li>
            </ul>
        </section>
    }
}
