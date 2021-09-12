use super::init::Terra;
use wasm_bindgen::prelude::*;

pub type WalletId = String;

impl Terra {
    pub fn login(&self, wallet_mnemonic:&str) -> WalletId{
        _login(wallet_mnemonic)
    }
}
#[wasm_bindgen(module = "/src/utils/terra/js/terra.js")]
extern "C" {
    fn _login(wallet_mnemonic: &str) -> WalletId;
}