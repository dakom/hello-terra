use wasm_bindgen::prelude::*;
use crate::config::{REMOTE_TARGET, RemoteTargetExt};
use once_cell::sync::Lazy;

pub const TERRA:Lazy<Terra> = Lazy::new(|| {
    Terra::new()
});

pub struct Terra {
}

impl Terra {
    pub fn new() -> Self {
        _init(&REMOTE_TARGET.terra_url(), &REMOTE_TARGET.terra_chain_id());

        Self {
        }
    }

}

#[wasm_bindgen(module = "/src/utils/terra/js/terra.js")]
extern "C" {
    fn _init(terra_url: &str, terra_chain_id: &str);
}