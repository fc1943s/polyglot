#![allow(special_module_name)]

pub mod app;
pub mod js;
pub mod state;
pub mod timer;

use leptos::{logging::log, *};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn app() -> Result<(), JsValue> {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    log!("csr mode - mounting to body");

    leptos::mount::mount_to_body(move || {
        view! { <crate::app::App /> }
    });

    Ok(())
}
