use serde::{Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use super::unwrap_ext::*;

#[wasm_bindgen(inline_js = "export function is_in_iframe() { return window && window.parent && window.location !== window.parent.location; }")]
extern "C" {
    pub fn is_in_iframe() -> bool;
}

pub trait IframeMsgSend: Serialize {
    fn try_post_message_to_top(&self) -> Result<(), JsValue> 
    {
        let window = web_sys::window().unwrap_ext();
        let top = window.top()?.unwrap_ext();

        top.post_message(&self.to_js_value(), "*")
    }
    fn try_post_message_to_parent(&self) -> Result<(), JsValue> 
    {
        let window = web_sys::window().unwrap_ext();
        let parent = window.parent()?.unwrap_ext();

        parent.post_message(&self.to_js_value(), "*")
    }
    fn try_post_to(&self, window:web_sys::Window) -> Result<(), JsValue> {
        window.post_message(&self.to_js_value(), "*")
    }

    fn to_js_value(&self) -> JsValue {
        serde_wasm_bindgen::to_value(self).unwrap_ext()
    }
    fn to_json_string(&self) -> String {
        serde_json::to_string(&self).unwrap_ext()
    }
}

pub trait IframeMsgRecv: DeserializeOwned {
    fn from_js_value(msg:JsValue) -> Self {
        serde_wasm_bindgen::from_value(msg).unwrap_ext()
    }
}