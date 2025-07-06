use generator::data::group_sigle::SigleGroup;
use leptos::prelude::*;

#[derive(Clone, Copy)]
pub struct SetModal(pub WriteSignal<Option<SigleGroup>>);

impl SetModal {
    pub fn from_context() -> WriteSignal<Option<SigleGroup>> {
        use_context::<Self>().unwrap().0
    }
}
