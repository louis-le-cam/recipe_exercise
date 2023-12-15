pub mod app;
mod cookies;
pub mod database;
mod model;
mod reactive_vec;

#[cfg(feature = "client")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();

    leptos::mount_to_body(app::App);
}
