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

use crate::config::{
    KNOWN_CONTRACT_CODE_IDS, 
    KNOWN_CONTRACT_HUB_ADDRS, 
    REMOTE_TARGET, 
    CONTRACT_CODE_ID_STORAGE,
    CONTRACT_ACCOUNT_HASH_URI, 
    CONTRACT_ACCOUNT_WASM_URI, 
    CONTRACT_HUB_HASH_URI, 
    CONTRACT_HUB_WASM_URI, 
    CONTRACT_HUB_ADDR_STORAGE, 
};
use cosmwasm_std::Addr;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::{
    collections::HashMap,
    fmt::{Debug,Error},
    cell::RefCell,
    rc::Rc,
};
use futures::join;
use dominator::clone;
use super::{prelude::*, storage::*, wallet_bridge::{ContractInstantiateMsg, ContractUploadMsg, PostError}}; 
use web_sys::{File, FileReader};
use gloo_events::EventListener;
use futures::channel::oneshot;
use js_sys::{ArrayBuffer, Uint8Array};
use awsm_web::loaders::fetch::{Response, fetch_url};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ContractInfo {
    pub hub_code_id: u64,
    pub hub_addr: String,
    pub account_code_id: u64,
    pub account_addr: String,
    pub chain_id: String,
}

pub fn init_known_contracts() {
    for (key, id) in KNOWN_CONTRACT_CODE_IDS.clone().into_iter() {
        set_contract_code_id(key, id);
    } 
    for (key, addr) in KNOWN_CONTRACT_HUB_ADDRS.clone().into_iter() {
        set_contract_hub_addr(key, addr);
    } 
}

//Keys have to be turned to strings for serde_json to work
//but it's just the serialized keys
pub type ContractCodeIdLookup = HashMap<String, u64>;
pub type ContractHubAddrLookup = HashMap<String, String>;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContractCodeIdLookupKey {
    pub chain_id: String,
    pub file_hash: String,
}

impl From<&ContractCodeIdLookupKey> for String {
    fn from(key:&ContractCodeIdLookupKey) -> String {
        serde_json::to_string(key).unwrap_ext()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ContractHubAddrLookupKey {
    pub chain_id: String,
    pub code_id: u64,
}
impl From<&ContractHubAddrLookupKey> for String {
    fn from(key:&ContractHubAddrLookupKey) -> String {
        serde_json::to_string(key).unwrap_ext()
    }
}

//ultimately returns (Hub Adddress, Account Address)
pub async fn bootstrap_registry(wallet_info: &WalletInfo) -> Result<ContractInfo, &'static str> {
    match load_code_ids(&wallet_info).await {
        Ok((hub_code_id, account_code_id)) => {
            let hub_addr = match get_contract_hub_addr(&ContractHubAddrLookupKey {
                chain_id: wallet_info.chain_id.clone(),
                code_id: hub_code_id.clone(),
            }) {
                Some(addr) => Some(addr),
                None => {
                    instantiate_hub_contract(wallet_info, hub_code_id, account_code_id).await
                }
            };

            if let Some(hub_addr) = hub_addr {
                let resp:Result<shared::contracts::hub::execute::AccountInfo, PostError> = ContractExecuteMsg{
                    addr: hub_addr.to_string(),
                    msg: shared::contracts::hub::execute::ExecuteMsg::GetAccountInfo,
                    coins: None 
                }.execute_bincode().await;

                match resp {
                    Ok(info) => Ok(
                        ContractInfo {
                            hub_code_id,
                            hub_addr,
                            account_code_id,
                            account_addr: info.contract_addr.to_string(),
                            chain_id: wallet_info.chain_id.to_string()
                        }
                    ),
                    _ => {
                        Err("couldn't instantiate account contract!")
                    }
                }
            } else {
                Err("couldn't instantiate hub contract!")
            }
        },
        _ => {
            Err("Couldn't upload WASM. Check your connection to the network?")
        }
    }
}

// (hub, account)
async fn load_code_ids(wallet_info: &WalletInfo) -> Result<(u64, u64), JsValue> {
    let hub_cache = get_hash_and_cache_code_id(wallet_info, CONTRACT_HUB_HASH_URI).await?;
    let account_cache = get_hash_and_cache_code_id(wallet_info, CONTRACT_ACCOUNT_HASH_URI).await?;

    match (hub_cache.1, account_cache.1) {
        (Some(hub_id), Some(account_id)) => {
            Ok((hub_id, account_id)) 
        },
        _ => { 
            //Seems to be a race condition of some sort using join!...
            let hub_id = upload_contract_remote(&wallet_info, &hub_cache.0, CONTRACT_HUB_WASM_URI).await
                    .ok_or(JsValue::from_str("unable to upload hub contract"))?;

            let account_id = upload_contract_remote(&wallet_info, &account_cache.0, CONTRACT_ACCOUNT_WASM_URI).await
                    .ok_or(JsValue::from_str("unable to upload account contract"))?;

            Ok((hub_id, account_id)) 
        }
    }
    
}

async fn get_hash_and_cache_code_id(wallet_info: &WalletInfo, hash_uri:&str) -> Result<(String, Option<u64>), JsValue> {
    let text = fetch_url(&format!("{}/{}", REMOTE_TARGET.media_url(), hash_uri))
        .await?
        .text()
        .await?;
    
    let hash = text.trim().to_string();

    let cached_code_id = get_contract_code_id(&ContractCodeIdLookupKey {
        file_hash: hash.clone(), 
        chain_id: wallet_info.chain_id.clone()
    });

    Ok((hash, cached_code_id))
}

pub fn get_contract_code_id(key: &ContractCodeIdLookupKey) -> Option<u64> {
    let key = String::from(key);

    let res = get_local_storage::<ContractCodeIdLookup>(CONTRACT_CODE_ID_STORAGE)
        .and_then(|lookup| {
            lookup.get(&key).map(|x| *x)
        });

    res
}
pub fn set_contract_code_id(key: ContractCodeIdLookupKey, id:u64) {
    if(crate::config::DEBUG.skip_cache_code_ids) {
        return;
    }
    let key = String::from(&key);

    let mut lookup = get_local_storage::<ContractCodeIdLookup>(CONTRACT_CODE_ID_STORAGE)
        .unwrap_or_default();
    
    lookup.insert(key, id);

    set_local_storage(CONTRACT_CODE_ID_STORAGE, lookup);

}
pub fn get_contract_hub_addr(key: &ContractHubAddrLookupKey) -> Option<String> {
    let key = String::from(key);

    get_local_storage::<ContractHubAddrLookup>(CONTRACT_HUB_ADDR_STORAGE)
        .and_then(|lookup| {
            lookup.get(&key).map(|x| x.to_string())
        })
}
pub fn set_contract_hub_addr(key:ContractHubAddrLookupKey, contract_addr:String) {
    if(crate::config::DEBUG.skip_cache_hub_addr) {
        return;
    }
    let key = String::from(&key);

    let mut lookup = get_local_storage::<ContractHubAddrLookup>(CONTRACT_HUB_ADDR_STORAGE)
        .unwrap_or_default();
    
    lookup.insert(key, contract_addr);

    set_local_storage(CONTRACT_HUB_ADDR_STORAGE, lookup);

}

pub async fn upload_contract_remote(wallet_info: &WalletInfo, file_hash:&String, file_uri:&str) -> Option<u64> {
    if let Ok(bytes) = load_contract_bytes(file_uri).await {
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

pub async fn instantiate_hub_contract(wallet_info: &WalletInfo, code_id: u64, account_code_id: u64) -> Option<String> {

    let addr = ContractInstantiateMsg { 
        id: code_id, 
        msg: Some(shared::contracts::hub::instantiate::InstantiateMsg::new(account_code_id))
    }.instantiate().await.ok().flatten();

    if let Some(addr) = addr { 
        let key = ContractHubAddrLookupKey {
            chain_id: wallet_info.chain_id.clone(),
            code_id: code_id.clone()
        };
        set_contract_hub_addr(key, addr.clone());
        Some(addr)
    } else {
        None
    }
}

async fn load_contract_bytes(uri:&str) -> Result<String, JsValue> {
    let buffer = fetch_url(&format!("{}/{}", REMOTE_TARGET.media_url(), uri))
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
            let key = ContractCodeIdLookupKey {
                chain_id: wallet_info.chain_id.clone(),
                file_hash: file_hash.clone(), 
            };
            set_contract_code_id(key, id);
            Some(id)
        },
        Err(err) => {
            None
        }
    }
}