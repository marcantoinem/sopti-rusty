#![feature(cmp_minmax)]
pub mod backend;
pub mod frontend;

cfg_if::cfg_if! { if #[cfg(feature = "hydrate")] {
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::frontend::app::App;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount::mount_to_body(App);
    }
}}
