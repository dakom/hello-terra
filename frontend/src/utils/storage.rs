use serde::{Serialize, de::DeserializeOwned};
use web_sys::{window, Storage};
use wasm_bindgen::prelude::*;
use super::unwrap_ext::*;

fn with_local_storage<F, A>(f:F) -> A 
where
    F: FnOnce(Storage) -> A
{

    let storage = window()
        .unwrap_ext()
        .local_storage()
        .unwrap_ext()
        .expect_ext("could not get local storage!");

    f(storage)
}

pub fn get_local_storage<A: DeserializeOwned>(storage_name:&str) -> Option<A> {
    with_local_storage(|storage| storage.get(storage_name).unwrap_ext())
        .and_then(|value| {
            serde_json::from_str(&value)
                .ok()
        })
}

pub fn set_local_storage<A: Serialize>(storage_name:&str, value:A) {
    let value = serde_json::to_string(&value).unwrap_ext();
    with_local_storage(|storage| storage.set(storage_name, &value).unwrap_ext());
}

pub fn delete_local_storage(storage_name:&str) { 
    with_local_storage(|storage| storage.remove_item(storage_name).unwrap_ext())
}

/*
pub fn load_csrf_token() -> Option<String> {
    let res = get_local_storage()
        .unwrap_ji()
        .get(CSRF_STORAGE_NAME)
        .unwrap_ji();

    if res.is_none() {
        log::warn!("unable to load CSRF!");
    }

    res
}

pub fn save_csrf_token(csrf:&str) {
    let local_storage = get_local_storage().unwrap_ji();

    local_storage.set(CSRF_STORAGE_NAME, csrf).unwrap_ji()
}

pub fn delete_csrf_token() -> Result<(), JsValue> {
    let local_storage = get_local_storage().unwrap_ji();

    local_storage.remove_item(CSRF_STORAGE_NAME)
}

pub fn _loca() -> Result<Storage, JsValue> {
}
*/