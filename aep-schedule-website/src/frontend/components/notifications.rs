use aep_schedule_generator::data::group_sigle::SigleGroup;
use leptos::*;

use crate::{
    backend::{
        routes::post_new_user_notification,
        shared::{email::Email, user_builder::UserBuilder},
    },
    frontend::components::icons::x::X,
};

#[component]
pub fn Notifications(
    modal: ReadSignal<Option<SigleGroup>>,
    set_modal: WriteSignal<Option<SigleGroup>>,
) -> impl IntoView {
    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let email = input_element().unwrap().value();
        if email.is_empty() {
            return;
        }
        let Some(group) = modal.get() else {
            return;
        };

        let user = UserBuilder::new(Email::new(email), group);

        spawn_local(async move {
            let _ = post_new_user_notification(user).await;
        });
    };

    let class = move || {
        let modal = modal.get();
        match modal {
            None => "notif-modal hidden",
            Some(_) => "notif-modal",
        }
    };

    view! {
        <div class=class>
            <div class="notif-body">
                <div class="close-button" on:click=move |_| {
                    set_modal.set(None);
                }>
                    <X size="2em"></X>
                </div>
                <form on:submit=on_submit>
                    <p>"Entrez votre courriel pour recevoir des notifications quand la section ouvrira"</p>
                    <input type="text"
                        pattern=r"[^@\s]+@[^@\s]+\.[^@\s]+"
                        node_ref=input_element
                    />
                    <input type="submit" value="Envoyer"/>
                </form>
            </div>
        </div>true
    }
}
