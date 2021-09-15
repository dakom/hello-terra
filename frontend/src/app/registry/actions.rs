use std::{cell::RefCell, fmt::Error, rc::Rc};
use super::state::*;
use web_sys::{File, FileReader};
use crate::utils::{prelude::*, storage::*};
use gloo_events::EventListener;
use dominator::clone;
use futures::channel::oneshot;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use js_sys::{ArrayBuffer, Uint8Array};
use awsm_web::loaders::fetch::{Response, fetch_url};
use crate::config::{REMOTE_TARGET, CONTRACT_ACCOUNT_HASH_URI, CONTRACT_ACCOUNT_WASM_URI, CONTRACT_ACCOUNT_ADDR_STORAGE, CONTRACT_ACCOUNT_ID_STORAGE};

impl Registry {
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
        let key = String::from(key);

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
        let key = String::from(key);

        let mut lookup = get_local_storage::<ContractAddrLookup>(CONTRACT_ACCOUNT_ADDR_STORAGE)
            .unwrap_or_default();
        
        lookup.insert(key, contract_addr);

        set_local_storage(CONTRACT_ACCOUNT_ADDR_STORAGE, lookup);

    }
    pub fn upload_contract_remote(state: Rc<Self>) {
        state.loader.load(clone!(state => async move {
            if let Ok(bytes) = Self::load_contract_bytes().await {
                Self::upload_contract(state.clone(), bytes).await;
            }
        }));
    }
    pub fn upload_contract_file(state: Rc<Self>, file: File) {
        state.loader.load(clone!(state => async move {
            let reader = FileReader::new().unwrap_ext();

            let (sender, receiver) = oneshot::channel::<Option<String>>();
            let sender = Rc::new(RefCell::new(Some(sender)));

            let listener = Rc::new(RefCell::new(None));
            
            *listener.borrow_mut() = Some(
                EventListener::new(&reader, "load", clone!(listener, sender, state => move |event| {
                    let reader:FileReader = event.target().unwrap_ext().unchecked_into();
                    let mut result:Option<String> = None;

                    if let Ok(value) = reader.result() {
                        let buffer:ArrayBuffer = value.unchecked_into();
                        result = Self::parse_wasm_bytes(buffer);
                    }

                    sender.borrow_mut().take().unwrap_ext().send(result);
                    listener.borrow_mut().take();
                }))
            );

            reader.read_as_array_buffer(&file);

            match receiver.await {
                Ok(result) => {
                    if let Some(byte_string) = result {
                        Self::upload_contract(state.clone(), byte_string).await;
                    } 
                },
                _ => { }
            }
        }));
    }

    pub fn instantiate_contract(state: Rc<Self>, contract_id: u64) {
        state.loader.load(clone!(state => async move {

            let (sender, receiver) = oneshot::channel::<Option<String>>();
            *state.contract_addr_sender.borrow_mut() = Some(sender);

            ContractInstantiateMsg { id: contract_id }
                .post();

            if let Some(addr) = receiver.await.ok().and_then(|result| result) {
                let key = ContractAddrLookupKey {
                    chain_id: state.wallet_info.chain_id.clone(),
                    wallet_addr: state.wallet_info.addr.clone(),
                    contract_id: contract_id.clone()
                };
                Self::set_contract_addr(key, addr.clone());
                state.app.contract_info.set_neq(Some(ContractInfo {
                    id: contract_id,
                    addr,
                    chain_id: state.wallet_info.chain_id.clone(),
                }));
            } else {
                web_sys::window().unwrap_ext().alert_with_message("unable to instantiate contract!");
            }
        }));
    }

    pub fn handle_wallet_message(state: Rc<Self>, msg: WalletMsg) {
        match msg {
            WalletMsg::Response(resp) => {
                match resp {
                    WalletResponse::ContractUpload(id) => {
                        if let Some(sender) = state.contract_id_sender.borrow_mut().take() {
                            sender.send(id);
                        }
                    },
                    WalletResponse::ContractInstantiate(addr) => {
                        if let Some(sender) = state.contract_addr_sender.borrow_mut().take() {
                            sender.send(addr);
                        }
                    },
                    _ => {}
                }
            },
            _ => {}

        }
    }

    async fn load_contract_bytes() -> Result<String, JsValue> {
        let buffer = fetch_url(&format!("{}/{}", REMOTE_TARGET.media_url(), CONTRACT_ACCOUNT_WASM_URI))
            .await?
            .array_buffer()
            .await?;
      
        match Self::parse_wasm_bytes(buffer) {
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

    async fn upload_contract(state: Rc<Self>, bytes:String) {
        let (sender, receiver) = oneshot::channel::<Option<u64>>();
        *state.contract_id_sender.borrow_mut() = Some(sender);
        WalletMsg::Request(WalletRequest::ContractUpload(bytes)).post();

        if let Some(id) = receiver.await.ok().and_then(|result| result) {
            let key = ContractIdLookupKey {
                chain_id: state.wallet_info.chain_id.clone(),
                contract_hash: state.contract_hash.get_cloned().expect_ext("contract hash must have existed before adding!")
            };
            Self::set_contract_id(key, id);
            state.contract_id.set_neq(Some(id));
        } else {
            web_sys::window().unwrap_ext().alert_with_message("unable to upload contract!");
        }
    }
}
