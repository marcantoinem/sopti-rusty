use leptos::*;

#[component]
pub fn Step(n: u8, title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <h2 class="small-margin">{n} - {title}</h2>
        <h5 class="small-margin">{description}</h5>
    }
}

#[component]
pub fn Todo() -> impl IntoView {
    view! {
        <Step n=1 title="Ajoutez vos cours" description="Utilisez la barre de recherche à gauche pour trouver et sélectionner vos cours. Une fois les cours sélectionnés, ils apparaîtront comme un onglet."/>
        <Step n=2 title="Forcer des heures libres (facultatif)" description="Sélectionnez une plage de temps à avoir absolument libre en pressant et relâchant sur votre horaire personnel."/>
        <Step n=3 title="Ouvrez des sections (facultatif)" description="Assurez d'avoir au moins une section d'ouverte pour la théorie et la pratique. En sélectionnant l'onglet du cours et en appuyant sur les sections."/>
        <Step n=4 title="Ajustez les paramètres (facultatif)" description="Bougez les curseurs en bas pour ajuster vos préférences. Vous pouvez choisir d'avoir plus de congés, de commencer en moyenne les cours plus tôt ou plus tard, ou de finir en moyenne plus tôt."/>
    }
}
