use serde::{Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::collections::HashMap;
use std::fmt::Debug;
use super::coin::Coins;
use super::contract::ContractExecuteMsg;
use super::unwrap_ext::MyUnwrapExt;
use super::wallet::{WalletPost};
use shared::execute::{
    summary::FullSummaryRequest
};

pub trait ExecutePostMsg {
    fn post(self, addr: &str, coins: Option<Coins>) where Self: Sized {
        self.try_post(addr, coins).unwrap_ext();
    }

    fn try_post(self, addr: &str, coins: Option<Coins>) -> Result<(), JsValue>;
}

impl ExecutePostMsg for shared::execute::ExecuteMsg {
    fn try_post(self, addr: &str, coins: Option<Coins>) -> Result<(), JsValue> {

        ContractExecuteMsg {
            addr: addr.to_string(),
            coins,
            msg: self 
        }.try_post()
    }
}