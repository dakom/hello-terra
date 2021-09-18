/*
    This file is about managing the local contract registry
    For interfacing with the contracts, see messages/contract.rs

    The registry is completely agnostic to the contracts themselves
    The mechanism is:

    Buildtime:
    1. Compile the contract
    2. Use Blake3 binary to generate a corresponding hash.txt
    
    Runtime:
    1. Populate LocalStorage with predefined known contracts (see config/contract.rs)
    2. When it comes time to init a contract on a given chain:
    3. check LocalStorage to see if we have the ContractInfo for a [file_hash + chain_id]
    4. If so, use it and return
    5. If not, check LocalStorage to see if we have the [code_id] for that same [file_hash + chain_id]
    6. If so, use it to instantiate a contract for the current user, and store/return the ContractInfo
    7. If not, download the .wasm, upload it to the network, and save the lookups for the above 
    8. Instantiate this ContractInfo and return

    Several of the steps can be interrupted via debug settings to allow a manual process
    
*/

use crate::config::{KNOWN_CONTRACT_IDS, KNOWN_CONTRACT_ADDRS, REMOTE_TARGET, CONTRACT_ACCOUNT_HASH_URI, CONTRACT_ACCOUNT_WASM_URI, CONTRACT_ACCOUNT_ADDR_STORAGE, CONTRACT_ACCOUNT_ID_STORAGE};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::{
    collections::HashMap,
    fmt::{Debug,Error},
    cell::RefCell,
    rc::Rc
};
use dominator::clone;
use super::{prelude::*, storage::*, wallet_bridge::{ContractInstantiateMsg, ContractUploadMsg}}; 
use web_sys::{File, FileReader};
use gloo_events::EventListener;
use futures::channel::oneshot;
use js_sys::{ArrayBuffer, Uint8Array};
use awsm_web::loaders::fetch::{Response, fetch_url};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ContractInfo {
    pub code_id: u64,
    pub addr: String,
    pub chain_id: String,
}

pub fn init_known_contracts() {
    for (key, id) in KNOWN_CONTRACT_IDS.clone().into_iter() {
        set_contract_id(key, id);
    } 
    for (key, addr) in KNOWN_CONTRACT_ADDRS.clone().into_iter() {
        set_contract_addr(key, addr);
    } 
}

//Keys have to be turned to strings for serde_json to work
//but it's just the serialized keys
pub type ContractIdLookup = HashMap<String, u64>;
pub type ContractAddrLookup = HashMap<String, String>;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContractIdLookupKey {
    pub chain_id: String,
    pub file_hash: String,
}

impl From<&ContractIdLookupKey> for String {
    fn from(key:&ContractIdLookupKey) -> String {
        serde_json::to_string(key).unwrap_ext()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContractAddrLookupKey {
    pub chain_id: String,
    pub code_id: u64,
    pub owner_addr: String
}

impl From<&ContractAddrLookupKey> for String {
    fn from(key:&ContractAddrLookupKey) -> String {
        serde_json::to_string(key).unwrap_ext()
    }
}


pub async fn load_contract_hash() -> Result<String, JsValue> {
    let text = fetch_url(&format!("{}/{}", REMOTE_TARGET.media_url(), CONTRACT_ACCOUNT_HASH_URI))
        .await?
        .text()
        .await?;
    
    Ok(text.trim().to_string())
}


pub fn get_contract_id(key: &ContractIdLookupKey) -> Option<u64> {
    let key = String::from(key);

    let res = get_local_storage::<ContractIdLookup>(CONTRACT_ACCOUNT_ID_STORAGE)
        .and_then(|lookup| {
            lookup.get(&key).map(|x| *x)
        });

    res
}
pub fn set_contract_id(key: ContractIdLookupKey, id:u64) {
    let key = String::from(&key);

    let mut lookup = get_local_storage::<ContractIdLookup>(CONTRACT_ACCOUNT_ID_STORAGE)
        .unwrap_or_default();
    
    lookup.insert(key, id);

    set_local_storage(CONTRACT_ACCOUNT_ID_STORAGE, lookup);

}
pub fn get_contract_addr(key: &ContractAddrLookupKey) -> Option<String> {
    let key = String::from(key);

    get_local_storage::<ContractAddrLookup>(CONTRACT_ACCOUNT_ADDR_STORAGE)
        .and_then(|lookup| {
            lookup.get(&key).map(|x| x.to_string())
        })
}
pub fn set_contract_addr(key:ContractAddrLookupKey, contract_addr:String) {
    let key = String::from(&key);

    let mut lookup = get_local_storage::<ContractAddrLookup>(CONTRACT_ACCOUNT_ADDR_STORAGE)
        .unwrap_or_default();
    
    lookup.insert(key, contract_addr);

    set_local_storage(CONTRACT_ACCOUNT_ADDR_STORAGE, lookup);

}
pub async fn upload_contract_remote(wallet_info: &WalletInfo, file_hash:&String, ) -> Option<u64> {
    if let Ok(bytes) = load_contract_bytes().await {
        upload_contract(wallet_info, file_hash, bytes).await
    } else {
        None
    }
}
pub async fn upload_contract_file(wallet_info: &WalletInfo, file_hash:&String, file: File) -> Option<u64> {
    let reader = FileReader::new().unwrap_ext();

    let (sender, receiver) = oneshot::channel::<Option<String>>();
    let sender = Rc::new(RefCell::new(Some(sender)));

    let listener = Rc::new(RefCell::new(None));
    
    *listener.borrow_mut() = Some(
        EventListener::new(&reader, "load", clone!(listener, sender => move |event| {
            let reader:FileReader = event.target().unwrap_ext().unchecked_into();
            let mut result:Option<String> = None;

            if let Ok(value) = reader.result() {
                let buffer:ArrayBuffer = value.unchecked_into();
                result = parse_wasm_bytes(buffer);
            }

            sender.borrow_mut().take().unwrap_ext().send(result);
            listener.borrow_mut().take();
        }))
    );

    reader.read_as_array_buffer(&file);

    match receiver.await {
        Ok(result) => {
            if let Some(byte_string) = result {
                upload_contract(wallet_info, file_hash, byte_string).await
            }  else {
                None
            }
        },
        _ => { None }
    }
}

pub async fn instantiate_contract<T: Serialize>(wallet_info: &WalletInfo, contract_id: u64, msg: Option<T>) -> Option<ContractInfo> {

    let addr = ContractInstantiateMsg { id: contract_id, msg}.instantiate().await.ok().flatten();

    if let Some(addr) = addr { 
        let key = ContractAddrLookupKey {
            chain_id: wallet_info.chain_id.clone(),
            owner_addr: wallet_info.addr.clone(),
            code_id: contract_id.clone()
        };
        set_contract_addr(key, addr.clone());
        Some(ContractInfo {
            code_id: contract_id,
            addr,
            chain_id: wallet_info.chain_id.clone(),
        })
    } else {
        web_sys::window().unwrap_ext().alert_with_message("unable to instantiate contract (check your network)!");
        None
    }
}

async fn load_contract_bytes() -> Result<String, JsValue> {
    let buffer = fetch_url(&format!("{}/{}", REMOTE_TARGET.media_url(), CONTRACT_ACCOUNT_WASM_URI))
        .await?
        .array_buffer()
        .await?;
    
    match parse_wasm_bytes(buffer) {
        Some(bytes) => Ok(bytes),
        None => Err(JsValue::NULL)
    }
}

fn parse_wasm_bytes(buffer:ArrayBuffer) -> Option<String> {
    let bytes = Uint8Array::new(&buffer).to_vec();
    if &bytes[0..4] == [0x00, 0x61, 0x73, 0x6D] {
        Some(base64::encode(&bytes))
    } else {
        web_sys::window().unwrap_ext().alert_with_message("invalid WASM file!");
        None
    }
}

async fn upload_contract(wallet_info: &WalletInfo, file_hash:&String, bytes:String) -> Option<u64> {

    match ContractUploadMsg(bytes).upload().await {
        Ok(id) => {
            let key = ContractIdLookupKey {
                chain_id: wallet_info.chain_id.clone(),
                file_hash: file_hash.clone(), 
            };
            set_contract_id(key, id);
            Some(id)
        },
        Err(err) => {
            web_sys::window().unwrap_ext().alert_with_message("unable to upload contract (check your network)!");
            None
        }
    }
}