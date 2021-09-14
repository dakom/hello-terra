use serde::{Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct ContractInfo {
    pub id: String,
    pub addr: String,
    pub chain_id: String,
}

type ContractHash = String;
type ContractId = String;
type WalletAddr = String;
type ContractAddr = String;

pub type ContractIdLookup = HashMap<ContractHash, ContractId>;
pub type ContractAddrLookup = HashMap<WalletAddr, ContractAddr>;
