/*
    The low-level mechanism for interacting with the iframe wallet manager

    The types in *this file and this file alone* must be kept in sync manually with the iframe side
    However, they are fairly generic - adding a new API only requires modifying these wrappers

    For typical use, just add types to messages.rs and call `let bar = ContractExecuteMsg {msg: Foo, ...}execute::<Bar>().await`

*/

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ContractExecuteMsg<T: Serialize> {
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
//TODO - Instantiation should also be here since it can receive dynamic data

/// heavy lifting below
use serde::{Serialize, Deserialize, de::DeserializeOwned};
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

// There's a lot of wrapping for convenience here... basically:
// ContractExecuteMsg can simply call .execute::<AnyResponseType>().await. Same idea for query
// At a lower level, any type can get .request() if it can impl Into<WalletBridgeRequest> (useful for common setup things)
// At a lower level still, any type can get .post() if it can impl Into<WalletBridgeMsg> (useful for bridge setup itself)
// And finally, wrapping anything that impls IFrameSend can wrap in WalletBridge
// Each of these has a try_ variant to allow handling low-level errors (though this is uncommon)
// For main top-level wrappers, a forget_ variant is also available to just send a message and not care about the response 
// This is used to deal with the case of any message at any time and doesn't make sense with the ID request/response system
// So it is not implemented for the business logic wrappers


pub const TAG:&'static str = "WALLET_BRIDGE";


//JSValues must be in thread_local
//It's pub only so app can set it
thread_local! {
    pub static WALLET_IFRAME:RefCell<Option<HtmlIFrameElement>> = RefCell::new(None);
}

//0 is reserved for system/setup messages
static MESSAGE_COUNTER:Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(1));


//The bridge
//All methods consume Self
pub struct WalletBridge <T: Serialize> {
    is_payload_first: bool,
    data: T,
    id: u64,
}

#[derive(thiserror::Error, Debug)]
pub enum PostError {
    #[error("post error: `{0}`")]
    String(String),

    #[error("(de)serialization error: `{0}`")]
    Serde(serde_wasm_bindgen::Error),

    #[error("iframe error: ?")]
    Iframe(JsValue) 
}



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

    pub async fn raw_post<R: DeserializeOwned + 'static>(mut self) -> R {
        self.try_raw_post().await.unwrap_ext().unwrap_ext()
    }

    //Post sends an iframe message
    //and then waits for the response for this ID
    //if the result is None, it means the request was cancelled somehow
    pub async fn try_raw_post<R: DeserializeOwned + 'static>(mut self) -> Option<Result<R, PostError>> {
        let window = web_sys::window().unwrap();
        let is_payload_first = self.is_payload_first;
        let self_id = self.id;

        let (sender, receiver) = oneshot::channel::<Result<R, PostError>>();

        let sender = RefCell::new(Some(sender));
        let listener = Rc::new(RefCell::new(None));

        *listener.borrow_mut() = Some(EventListener::new(&window, "message", clone!(listener => move |evt| {
            let evt:web_sys::MessageEvent = evt.clone().unchecked_into();

            //hehe... https://github.com/terra-money/terra.js/issues/133
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

        self.try_raw_post_forget().ok()?;

        receiver.await.ok()
    }

    //Simple - post to iframe and forget about the response
    //Only used for initial setup really
    pub fn try_raw_post_forget(self) -> Result<(), PostError> {
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


//Top-level container messages for known static types
//Once we require dynamic types (like inner contract message)
//It needs it's own top-level wrapper again
#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "kind", content = "data")]
pub enum WalletBridgeMsg {
    /// Sent from the Iframe to indicate current status
    #[serde(rename = "wallet_bridge_status")]
    Status(WalletBridgeStatus),
    
    /// Sent from the iframe for window events
    #[serde(rename = "wallet_bridge_window_event")]
    WindowEvent(WalletBridgeWindowEvent),
    
    /// Sent from the app to for wallet requests
    #[serde(rename = "wallet_bridge_request")]
    Request(WalletBridgeRequest),
    
    /// Sent from the iframe for wallet responses
    #[serde(rename = "wallet_bridge_response")]
    Response(WalletBridgeResponse),

}

impl From<WalletBridgeMsg> for WalletBridge<WalletBridgeMsg> {
    fn from(from: WalletBridgeMsg) -> Self {
        Self::new(from)
    }
}

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum WalletBridgeStatus {
    Initializing,
    Wallet_Not_Connected,
    Wallet_Connected 
}

impl From<WalletBridgeStatus> for WalletBridgeMsg {
    fn from(from: WalletBridgeStatus) -> Self {
        WalletBridgeMsg::Status(from).into()
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "kind", content = "data")]
pub enum WalletBridgeRequest {
    Setup(WalletBridgeSetup),
    WalletInfo,
    ContractUpload(String),
}

impl From<WalletBridgeRequest> for WalletBridgeMsg {
    fn from(from: WalletBridgeRequest) -> Self {
        WalletBridgeMsg::Request(from).into()
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "kind", content = "data")]
pub enum WalletBridgeResponse {
    WalletInfo(Option<WalletInfo>),
    ContractUpload(Option<u64>),
}

impl From<WalletBridgeResponse> for WalletBridgeMsg {
    fn from(from: WalletBridgeResponse) -> Self {
        WalletBridgeMsg::Response(from).into()
    }
}

// bridge-specific request/response data types

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "kind", content = "data")]
pub enum WalletBridgeSetup{
    ConnectExtension,
    ConnectMobile,
    Install,
    Disconnect,
    ConnectManually(String, String, String)
}
 
impl From<WalletBridgeSetup> for WalletBridgeMsg {
    fn from(from: WalletBridgeSetup) -> Self {
        WalletBridgeMsg::Request(WalletBridgeRequest::Setup(from)).into()
    }
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




/// Contract wrapping

//required for serde to wrap it
//not required for RECV because we know it from the ID
//for this reason, the _forget variants aren't implemented (it would be confusing)
#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "kind", content = "data")]
pub enum ContractWrapper<T: Serialize> {
    #[serde(rename = "contract_instantiate")]
    Instantiate(ContractInstantiateMsg<T>),
    #[serde(rename = "contract_execute")]
    Execute(ContractExecuteMsg<T>),
}


impl <T: Serialize> ContractInstantiateMsg<T> {
    fn wrap(self) -> ContractWrapper<T> {
        ContractWrapper::Instantiate(self)
    } 

    pub async fn instantiate(self) -> Option<String> {
        self.try_instantiate().await.unwrap_ext().unwrap_ext()
    }
    pub async fn try_instantiate(self) -> Option<Result<Option<String>, PostError>> {
        WalletBridge::new(self.wrap()).try_raw_post::<Option<String>>().await
    }
}

impl <T: Serialize> ContractExecuteMsg<T> {
    fn wrap(self) -> ContractWrapper<T> {
        ContractWrapper::Execute(self)
    } 

    pub async fn execute<R: DeserializeOwned + 'static>(self) -> R {
        self.try_execute().await.unwrap_ext().unwrap_ext()

    }
    pub async fn try_execute<R: DeserializeOwned + 'static>(self) -> Option<Result<R, PostError>> {
        let mut bridge = WalletBridge::new(self.wrap());
        bridge.is_payload_first = true;

        bridge.try_raw_post::<R>().await

    }
}

//Some basic impls
#[async_trait(?Send)]
pub trait WalletBridgeRequestExt {
    async fn try_request(mut self) -> Option<Result<WalletBridgeResponse, PostError>>;
    async fn request(mut self) -> WalletBridgeResponse;
}
#[async_trait(?Send)]
impl <T: Into<WalletBridgeRequest>> WalletBridgeRequestExt for T {
    async fn request(mut self) -> WalletBridgeResponse {
        self.try_request().await.unwrap_ext().unwrap_ext()
    }
    async fn try_request(mut self) -> Option<Result<WalletBridgeResponse, PostError>> {
        let resp = WalletBridgeMsg::Request(self.into())
            .try_post::<WalletBridgeMsg>()
            .await;

        match resp {
            None => None,
            Some(resp_err) => {
                match resp_err {
                    Err(err) => Some(Err(err)),
                    Ok(resp_ok) => {
                        match resp_ok {
                            WalletBridgeMsg::Response(resp) => {
                                Some(Ok(resp))
                            },
                            _ => None
                        }
                    }
                }
            }
        }
    }
}

#[async_trait(?Send)]
pub trait WalletBridgeExt {
    async fn post<R: DeserializeOwned + 'static>(mut self) -> R;
    async fn try_post<R: DeserializeOwned + 'static>(mut self) -> Option<Result<R, PostError>>;
    fn try_post_forget(self) -> Result<(), PostError>;
}

#[async_trait(?Send)]
impl <T: Into<WalletBridgeMsg>> WalletBridgeExt for T {
    async fn post<R: DeserializeOwned + 'static>(mut self) -> R {
        self.try_post().await.unwrap_ext().unwrap_ext()
    }
    async fn try_post<R: DeserializeOwned + 'static>(mut self) -> Option<Result<R, PostError>> {
        WalletBridge::from(self.into()).try_raw_post().await
    }

    fn try_post_forget(self) -> Result<(), PostError> {
        WalletBridge::from(self.into()).try_raw_post_forget()
    }
}
