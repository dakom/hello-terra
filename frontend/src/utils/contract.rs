use serde::{Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::collections::HashMap;
use std::fmt::Debug;
use super::coin::Coins;
use super::unwrap_ext::MyUnwrapExt;
use super::wallet::{WalletMsg, WalletRequest, WalletPost, WalletPostRef};


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ContractInstantiateMsg {
    pub id: u64
}

impl WalletPost for ContractInstantiateMsg {
    fn try_post(self) -> Result<(), JsValue> {
        WalletMsg::Request(WalletRequest::ContractInstantiate(self))
            .try_post()
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ContractExecuteMsg {
    pub addr: String,
    pub msg: ContractExecuteMsgContent, 
    pub coins: Option<Coins>
}

impl WalletPost for ContractExecuteMsg {
    fn try_post(self) -> Result<(), JsValue> {
        WalletMsg::Request(WalletRequest::ContractExecute(self))
            .try_post()
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "kind", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum ContractExecuteMsgContent {
    FullSummaryRequest(shared::summary::FullSummaryRequest)
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "kind", content = "data")]
#[serde(rename_all = "snake_case")]
pub enum ContractExecuteResp {
    FullSummaryResponse(shared::summary::FullSummaryResponse)
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ContractInfo {
    pub id: u64,
    pub addr: String,
    pub chain_id: String,
}

//Keys have to be turned to strings for serde_json to work
pub type ContractIdLookup = HashMap<String, u64>;
pub type ContractAddrLookup = HashMap<String, String>;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContractIdLookupKey {
    pub chain_id: String,
    pub contract_hash: String,
}

impl From<ContractIdLookupKey> for String {
    fn from(key:ContractIdLookupKey) -> String {
        (&key).into()
    }
}
impl From<&ContractIdLookupKey> for String {
    fn from(key:&ContractIdLookupKey) -> String {
        serde_json::to_string(key).unwrap_ext()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContractAddrLookupKey {
    pub chain_id: String,
    pub contract_id: u64,
    pub wallet_addr: String
}
impl From<ContractAddrLookupKey> for String {
    fn from(key:ContractAddrLookupKey) -> String {
        (&key).into()
    }
}
impl From<&ContractAddrLookupKey> for String {
    fn from(key:&ContractAddrLookupKey) -> String {
        serde_json::to_string(key).unwrap_ext()
    }
}