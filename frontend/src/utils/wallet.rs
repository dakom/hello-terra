/// A messaging system for interacting with the iframe wallet manager
/// The types must be kept in sync manually with the React/iframe side
/// However, they are fairly generic - adding new contract message payloads is purely in Rust

use serde::{Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlIFrameElement;
use std::cell::RefCell;
use super::contract::{ContractInstantiateMsg, ContractExecuteMsg, ContractExecuteResp};
use super::{iframe::*, unwrap_ext::*, coin::*};
thread_local! {
    pub static WALLET_IFRAME:RefCell<Option<HtmlIFrameElement>> = RefCell::new(None);
}

impl IframeMsgRecv for WalletMsg {}
impl IframeMsgSend for WalletMsg {}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "kind", content = "data")]
pub enum WalletMsg {
    /// Sent from the Iframe to indicate current status
    #[serde(rename = "wallet_status")]
    Status(WalletStatus),
    /// Sent from the app to run setup commands
    #[serde(rename = "wallet_setup")]
    Setup(WalletSetup),
    /// Sent from the iframe for window events
    #[serde(rename = "wallet_window")]
    Window(WalletWindowEvent),
    /// Sent from the app to for wallet requests
    #[serde(rename = "wallet_request")]
    Request(WalletRequest),
    /// Sent from the iframe for wallet responses
    #[serde(rename = "wallet_response")]
    Response(WalletResponse),
}

pub trait WalletPost {
    fn post(self) where Self: Sized {
        self.try_post().unwrap_ext();
    }

    fn try_post(self) -> Result<(), JsValue>;
}

pub trait WalletPostRef {
    fn post(&self) {
        self.try_post().unwrap_ext();
    }

    fn try_post(&self) -> Result<(), JsValue>;
}

impl WalletPostRef for WalletMsg {
    fn try_post(&self) -> Result<(), JsValue> {
        WALLET_IFRAME.with(|iframe| {
            match iframe.borrow().as_ref() {
                None => Err(JsValue::from_str("Iframe doesn't exist yet!")),
                Some(iframe) => {
                    self.try_post_to(iframe.content_window().unwrap_ext())
                }
            }
        })
    }
}



#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum WalletStatus {
    Initializing,
    Wallet_Not_Connected,
    Wallet_Connected 
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WalletSetup{
    ConnectExtension,
    ConnectMobile,
    Install,
    Disconnect
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "kind", content = "data")]
pub enum WalletRequest {
    Addr,
    ContractUpload(String),
    ContractInstantiate(ContractInstantiateMsg),
    ContractExecute(ContractExecuteMsg),
}


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "kind", content = "data")]
pub enum WalletResponse {
    Addr(Option<WalletInfo>),
    ContractUpload(Option<u64>),
    ContractInstantiate(Option<String>),
    ContractExecute(Option<ContractExecuteResp>),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub struct WalletInfo {
    pub addr: String,
    pub chain_id: String,
    pub network_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum WalletWindowEvent {
    Click,
}