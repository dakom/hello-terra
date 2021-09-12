use super::init::Terra;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use crate::utils::prelude::*;

pub type WalletId = String;

impl Terra {
    //bytes is as base64 string
    pub async fn upload_contract(&self, bytes:&str) -> Result<String, String> {
        JsFuture::from(_upload_contract(bytes)).await
            .map(|value| value.as_string().unwrap_ext())
            .map_err(|value| value.as_string().unwrap_ext())
    }
}
#[wasm_bindgen(module = "/src/utils/terra/js/terra.js")]
extern "C" {
    fn _upload_contract(bytes: &str) -> js_sys::Promise;
}