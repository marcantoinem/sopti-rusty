use leptos::*;

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <section>
            <h1>"Générateur d'horaire de l'AEP v2"</h1>
            <p>
                "Vous en avez assez d'avoir des horaires horribles fait pas le registrariat? Vous êtes au bon endroit. Le générateur d'horaire v2 est une réécriture plus performante, intuive et moderne du générateur d'horaire. Il vous aidera à trouver un horaire parfait."
            </p>
            <h2>"Auteurs"</h2>
            <span style="bold">"Marc-Antoine Manningham"</span>
        </section>
    }
}
