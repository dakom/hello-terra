use serde::Deserialize;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

//re-export dominator::events
pub use dominator::events::*;

// new events can be added too:
// use dominator_helpers::{temp_make_event, make_custom_event_serde, make_custom_event};
// temp_make_event!(Open, "open" => web_sys::Event);
// temp_make_event!(Close, "close" => web_sys::Event);

// // Custom Bounds 
// #[derive(Deserialize, Debug)]
// pub struct CustomBoundsData {
//     pub x: f64,
//     pub y: f64,
//     pub width: f64,
//     pub height: f64,
// }

// make_custom_event_serde!("custom-bounds", CustomBounds, CustomBoundsData);
