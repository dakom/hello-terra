/*
    The low-level mechanism for interacting with the iframe wallet manager

    The types in *this file and this file alone* must be kept in sync manually with the iframe side
    However, they are fairly generic - adding a new API only requires modifying these wrappers

    For typical use, just add types to messages.rs and call `let bar = ContractExecuteMsg {msg: Foo, ...}execute::<Bar>().await`

    The Response types do not need wrappers because we pass a reference ID and check a shared tag to match it to the request

*/


///////// DATA STRUCTS ///////////////
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ContractExecuteMsg<T> {
    pub addr: String,
    pub msg: T, 
    //pub msg: shared::execute::ExecuteMsg, 
    pub coins: Option<Coins>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ContractInstantiateMsg<T> {
    pub id: u64,
    pub msg: Option<T>
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ContractQueryMsg<T> {
    pub addr: String,
    pub msg: T
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ContractUploadMsg(pub String);

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "kind", content = "data")]
pub enum WalletBridgeSetupRequest {
    ConnectExtension,
    ConnectMobile,
    Install,
    Disconnect,
    ConnectManually(String, String, String),
    WalletInfo,
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
pub enum WalletBridgeWindowEvent {
    Click,
}

#[derive(thiserror::Error, Debug)]
pub enum PostError {
    #[error("post error: `{0}`")]
    String(String),

    #[error("(de)serialization error: `{0}`")]
    Serde(serde_wasm_bindgen::Error),

    #[error("iframe error: ?")]
    Iframe(JsValue),

    #[error("Cancelled")]
    Cancelled
}

impl PostError {
    pub fn is_cancelled(&self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(&PostError::Cancelled)
    }
}

///// INTERNAL WRAPPERS FOR SERDE ///////////////////

//Top-level container messages for known static types
//Once we require dynamic types (like inner contract message)
//It needs it's own top-level wrapper again
//We do not need to make wrappers for the return type, except when it
//is OOB-driven (like window events and wallet status)
#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "kind", content = "data")]
pub enum WalletBridgeMsgWrapper<T> {
    /// Sent from the Iframe to indicate current status
    /// Can happen at any time
    #[serde(rename = "status")]
    Status(WalletBridgeStatus),
    
    /// Various triggers and requests to set things up
    /// From app to bridge
    #[serde(rename = "setup_request")]
    SetupRequest(WalletBridgeSetupRequest),
    
    /// Sent from the iframe for window events
    /// Theoretically any time but really just for QR code
    #[serde(rename = "window_event")]
    WindowEvent(WalletBridgeWindowEvent),

    #[serde(rename = "contract_instantiate")]
    ContractInstantiate(ContractInstantiateMsg<T>),

    #[serde(rename = "contract_execute")]
    ContractExecute(ContractExecuteMsg<T>),

    #[serde(rename = "contract_query")]
    ContractQuery(ContractQueryMsg<T>),

    #[serde(rename = "contract_upload")]
    ContractUpload(ContractUploadMsg),
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum WalletBridgeStatus {
    Initializing,
    Wallet_Not_Connected,
    Wallet_Connected 
}



///////// IMPLEMENTATION AND INTERNAL WRAPPERS ///////////////
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use serde_json::json;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlIFrameElement;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};
use dominator::clone;
use gloo_events::EventListener;
use once_cell::sync::Lazy;
use futures::channel::oneshot;
use std::future::Future;
use super::{unwrap_ext::*, coin::*};
use async_trait::async_trait;

//JSValues must be in thread_local
//It's pub only so app can set it
thread_local! {
    pub static WALLET_IFRAME:RefCell<Option<HtmlIFrameElement>> = RefCell::new(None);
}

//Helps ensure we're talking to our system
pub const TAG:&'static str = "WALLET_BRIDGE";

//0 is reserved for system/setup messages that never get waited on, so start at 1
static MESSAGE_COUNTER:Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(1));

//The bridge
//All methods consume Self
pub struct WalletBridge <T: Serialize> {
    is_payload_first: bool,
    data: T,
    id: u64,
}

/// Main implementation. Everything else builds on this
impl <T: Serialize> WalletBridge <T> {
    pub fn new(data:T) -> Self {
        let id = MESSAGE_COUNTER.load(Ordering::SeqCst);
        MESSAGE_COUNTER.store(id+1, Ordering::SeqCst);
        Self {
            is_payload_first: false,
            data,
            id
        }
    }

    //Post sends an iframe message
    //and then waits for the response for this ID
    //if the result is None, it means the request was cancelled somehow
    pub async fn post<R: DeserializeOwned + 'static>(mut self) -> Result<R, PostError> {
        let window = web_sys::window().unwrap();
        let is_payload_first = self.is_payload_first;
        let self_id = self.id;

        let (sender, receiver) = oneshot::channel::<Result<R, PostError>>();

        let sender = RefCell::new(Some(sender));
        let listener = Rc::new(RefCell::new(None));

        *listener.borrow_mut() = Some(EventListener::new(&window, "message", clone!(listener => move |evt| {
            let evt:web_sys::MessageEvent = evt.clone().unchecked_into();

            //hehe... https://github.com/terra-money/terra.js/issues/133
            //TODO - at least flatten this a bit
            let msg = if is_payload_first {
                let msg = serde_wasm_bindgen::from_value::<(u64, String, String)>(evt.data());
                match msg {
                    Ok((id, tag, data)) => {
                        match base64::decode(data) {
                            Ok(data) => {
                                bincode::deserialize::<R>(&data)
                                    .map(|msg| (id, tag, msg))
                                    .map_err(|err| format!("{:?}", err))
                            },
                            Err(err) => {
                                Err(format!("{:?}", err))
                            }
                        }
                    }
                    Err(err) => Err(format!("{:?}", err))
                }
            } else {
                serde_wasm_bindgen::from_value::<(u64, String, R)>(evt.data()) 
                    .map_err(|err| format!("{:?}", err))
            };
            
            let msg = match msg {
                Ok((id, tag, res)) => {
                    if tag != TAG {
                        log::info!("Got message for different iframe, ignoring!");
                        None
                    } else if id == self_id {
                        Some(Ok(res))
                    } else {
                        log::info!("Got message for different id, ignoring!");
                        None
                    }
                },
                Err(err) => {
                    Some(Err(PostError::String(err)))
                }
            };

            if let Some(msg) = msg {
                if let Some(sender) = sender.borrow_mut().take() {
                    sender.send(msg);
                }

                let _ = listener.borrow_mut().take();
            }
        })));

        self.post_forget()?;

        match receiver.await {
            Err(_) => Err(PostError::Cancelled),
            Ok(res) => res
        }
    }

    //Simple - post to iframe and forget about the response
    //Only used for initial setup really
    pub fn post_forget(self) -> Result<(), PostError> {
        WALLET_IFRAME.with(|iframe| {
            match iframe.borrow().as_ref() {
                None => Err(JsValue::from_str("Iframe doesn't exist yet!")),
                Some(iframe) => {

                    let data = serde_wasm_bindgen::to_value(&(self.id, TAG, self.data)).unwrap_ext();

                    iframe.content_window().unwrap_ext().post_message(&data, "*")
                }
            }
        })
        .map_err(|err| PostError::Iframe(err))
    }
}

/// Contract wrapping

impl <T: Serialize> ContractExecuteMsg<T> {
    fn wrap(self) -> WalletBridgeMsgWrapper<T> {
        WalletBridgeMsgWrapper::ContractExecute(self)
    } 

    pub async fn execute<R: DeserializeOwned + 'static>(self) -> Result<R, PostError> {
        let mut bridge = WalletBridge::new(self.wrap());
        bridge.is_payload_first = true;

        bridge.post::<R>().await

    }
}

impl <T: Serialize> ContractQueryMsg<T> {
    fn wrap(self) -> WalletBridgeMsgWrapper<T> {
        WalletBridgeMsgWrapper::ContractQuery(self)
    } 

    pub async fn query<R: DeserializeOwned + 'static>(self) -> Result<R, PostError> {
        let mut bridge = WalletBridge::new(self.wrap());
        //bridge.is_payload_first = true;
        bridge.post::<R>().await
    }
}

impl <T: Serialize> ContractInstantiateMsg<T> {
    fn wrap(self) -> WalletBridgeMsgWrapper<T> {
        WalletBridgeMsgWrapper::ContractInstantiate(self)
    } 

    pub async fn instantiate(self) -> Result<Option<String>, PostError> {
        WalletBridge::new(self.wrap()).post::<Option<String>>().await
    }
}

impl ContractUploadMsg {
    fn wrap(self) -> WalletBridgeMsgWrapper<()> {
        WalletBridgeMsgWrapper::ContractUpload(self)
    } 

    pub async fn upload(self) -> Result<u64, PostError> {
        WalletBridge::new(self.wrap())
            .post::<u64>().await
    }
}


/// Top-level request wrapper (also for one-offs)
impl WalletBridgeSetupRequest {
    fn wrap(self) -> WalletBridgeMsgWrapper<()> {
        WalletBridgeMsgWrapper::SetupRequest(self)
    }

    pub async fn request<R: DeserializeOwned + 'static>(mut self) -> Result<R, PostError> {
        WalletBridge::new(self.wrap()).post().await
    }
    pub fn request_forget(self) -> Result<(), PostError> {
        WalletBridge::new(self.wrap()).post_forget()
    }
}
