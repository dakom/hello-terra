#![allow(warnings)]
#![feature(async_closure)]
#![feature(never_type)]

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod app;
mod components;
mod utils;
mod config;

use wasm_bindgen::prelude::*;
use cfg_if::cfg_if;
use app::App;

#[wasm_bindgen(start)]
pub async fn main_js() {
    setup_logger();
    config::init_stylesheet();
    utils::contract::init_known_contracts();
    dominator::append_dom(&dominator::body(), App::render(App::new())); 

    /*
    let foo:Result<u32, String> = Ok(42);
    let bar:Result<u32, String> = Err("forty-two".to_string());

    web_sys::console::log_1(&serde_wasm_bindgen::to_value(&foo).unwrap());
    web_sys::console::log_1(&serde_wasm_bindgen::to_value(&bar).unwrap());
    */

}

// enable logging and panic hook only in dev mode
cfg_if! {
    if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook"))] {
        fn setup_logger() {
            wasm_logger::init(wasm_logger::Config::default());
            console_error_panic_hook::set_once();
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            log::info!("rust logging enabled!!!");
        }
    } else {
        fn setup_logger() {
            log::info!("rust logging disabled!"); //<-- won't be seen
        }
    }
}


