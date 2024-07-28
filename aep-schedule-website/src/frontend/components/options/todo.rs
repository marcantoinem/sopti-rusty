use std::cmp::Ordering;

use aep_schedule_generator::algorithm::{generation::SchedulesOptions, schedule::Schedule};
use leptos::*;

use crate::frontend::{
    components::options::state::OptionState, pages::generator::FirstGenerationDone,
};

#[component]
pub fn Step(
    n: u8,
    step: ReadSignal<u8>,
    title: &'static str,
    description: &'static str,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let bg_color = move || {
        match n.cmp(&step.get()) {
            Ordering::Less => "flex transition-all items-center justify-center w-10 h-10 border rounded-full bg-green-400",
            Ordering::Greater => "flex transition-all items-center justify-center w-10 h-10 border rounded-full bg-gray-100",
            Ordering::Equal => "flex transition-all items-center justify-center w-10 h-10 border rounded-full bg-amber-400",
        }
    };

    view! {
        <div class="flex">
            <div class="flex flex-col items-center mr-4">
              <div>
                <div class=bg_color>
                  <svg class="w-4 text-gray-600" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" viewBox="0 0 24 24">
                    <line fill="none" stroke-miterlimit="10" x1="12" y1="2" x2="12" y2="22"></line>
                    <polyline fill="none" stroke-miterlimit="10" points="19,15 12,22 5,15"></polyline>
                  </svg>
                </div>
              </div>
            <div class="w-px h-full bg-gray-300"></div>
        </div>
        <div class="pt-1 pb-8">
          <p class="mb-2 text-lg font-bold">{n}" - "{title}</p>
          <p class="text-gray-700">
            {description}
          </p>
          {children.map(|c| c())}
        </div>
      </div>
    }
}

#[component]
pub fn Todo(
    action: Action<SchedulesOptions, Vec<Schedule>>,
    step: ReadSignal<u8>,
    section_error: RwSignal<String>,
    personal_error: RwSignal<String>,
) -> impl IntoView {
    let state: OptionState = use_context().unwrap();
    let first_generation_done: FirstGenerationDone = use_context().unwrap();

    let submit = move |_| {
        first_generation_done.0.set(true);
        action.dispatch((&state).into())
    };

    let disab = move || {
        let step = step.get();
        match step {
            0..=4 => "disabled",
            _ => "",
        }
    };

    view! {
        <div class="px-4 py-4 mx-auto">
            <div class="grid gap-6 row-gap-10">
                <div class="lg:py-6 lg:pr-16">
                    <Step n=1 step title="Ajoutez vos cours" description="Utilisez la barre de recherche à gauche pour trouver et sélectionner vos cours. Une fois les cours sélectionnés, ils apparaîtront comme un onglet."/>
                    <Step n=2 step title="Ouvrez des sections" description="Assurez d'avoir au moins une section d'ouverte pour la théorie et la pratique. En sélectionnant l'onglet du cours et en appuyant sur les sections.">
                        <span class="text-red-800">{section_error}</span>
                    </Step>
                    <Step n=3 step title="Forcer des heures libres" description="Sélectionnez une plage de temps à avoir absolument libre en pressant et relâchant sur votre horaire personnel.">
                        <span class="text-red-800">{personal_error}</span>
                    </Step>
                    <Step n=4 step title="Ajustez les paramètres" description="Bougez les curseurs en bas pour ajuster vos préférences. Vous pouvez choisir d'avoir plus de congés, de commencer en moyenne les cours plus tôt ou plus tard, ou de finir en moyenne plus tôt."/>
                    <div class="flex items-center">
                        <div class="flex flex-col items-center mr-4">
                            <div>
                                <div class="flex transition-colors items-center justify-center w-10 h-10 border rounded-full" class=("bg-gray-100", move || step.get() != 5) class=("bg-green-400", move || step.get() == 5)>
                                    <svg class="w-6 text-gray-600" stroke="currentColor" viewBox="0 0 24 24">
                                        <polyline fill="none" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" stroke-miterlimit="10" points="6,12 10,16 18,8"></polyline>
                                    </svg>
                                </div>
                            </div>
                        </div>
                        <button on:click=submit class="select-none rounded-lg bg-amber-500 py-1 text-lg font-sans font-semibold px-2 w-64 self-center text-center align-middle text-black shadow-md shadow-amber-500/20 transition-all hover:shadow-lg hover:shadow-amber-500/40 focus:opacity-[0.85] focus:shadow-none active:opacity-[0.85] active:shadow-none disabled:pointer-events-none disabled:opacity-50 disabled:shadow-none" prop:disabled=disab>"Générer les horaires"</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
