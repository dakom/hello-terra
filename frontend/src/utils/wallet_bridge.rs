/*
    The high-level mechanism for interacting with the iframe wallet manager
    
    For the messages themselves, see messages/wallet.rs

    The types must be kept in sync manually with the React/iframe side
    However, they are fairly generic - adding a new API only requires
    The very high-level wrappers (excute *any* contract, for example)

    Also included are impls for .into::<WalletBridge<WalletBridgeMsg>>()
*/

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
use super::{iframe::*, unwrap_ext::*, coin::*};
use async_trait::async_trait;

// Any type can get request() if it can impl Into<WalletBridgeRequest>
// And any type can get post() if it can impl Into<WalletBridgeMsg>

// The top-level types have impls below, so for example it's simple to just call
// `WalletBridgeSetup::ConnectMobile.post_forget()` to just fire it off, or .post() for a proper future
// more types (like contract execution) can easily be impl'd to get request() via a macro
// since it only needs to satisfy `Into<WalletBridgeRequest>` (which is easily doable for any serde type)

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
        WalletBridge::from(self.into()).try_post().await
    }

    fn try_post_forget(self) -> Result<(), PostError> {
        WalletBridge::from(self.into()).try_post_forget()
    }
}


//JSValues must be in thread_local
//It's pub only so app can set it
thread_local! {
    pub static WALLET_IFRAME:RefCell<Option<HtmlIFrameElement>> = RefCell::new(None);
}

//0 is reserved for system/setup messages
static MESSAGE_COUNTER:Lazy<AtomicU64> = Lazy::new(|| AtomicU64::new(1));

//For serializing our message wrappers with the ID
impl <T: IframeMsgSend> IframeMsgSend for (u64, T) {}

//The bridge
//All methods consume Self
pub struct WalletBridge <T: IframeMsgSend> {
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

impl <T: IframeMsgSend> WalletBridge <T> {
    pub fn new(data:T) -> Self {
        let id = MESSAGE_COUNTER.load(Ordering::SeqCst);
        MESSAGE_COUNTER.store(id+1, Ordering::SeqCst);
        Self {
            data,
            id
        }
    }

    pub async fn post<R: DeserializeOwned + 'static>(mut self) -> R {
        self.try_post().await.unwrap_ext().unwrap_ext()
    }

    //Post sends an iframe message
    //and then waits for the response for this ID
    //if the result is None, it means the request was cancelled somehow
    pub async fn try_post<R: DeserializeOwned + 'static>(mut self) -> Option<Result<R, PostError>> {
        let window = web_sys::window().unwrap();
        let self_id = self.id;

        let (sender, receiver) = oneshot::channel::<Result<R, PostError>>();

        let sender = RefCell::new(Some(sender));
        let listener = Rc::new(RefCell::new(None));

        *listener.borrow_mut() = Some(EventListener::new(&window, "message", clone!(listener => move |evt| {
            log::info!("got message");
            let evt:web_sys::MessageEvent = evt.clone().unchecked_into();
            let msg = match serde_wasm_bindgen::from_value::<(u64, R)>(evt.data()) {
                Ok((id, res)) => {
                    if id == self_id {
                        Some(Ok(res))
                    } else {
                        log::info!("Got message for different id, ignoring!");
                        None
                    }
                },
                Err(err) => {
                    Some(Err(PostError::Serde(err)))
                }
            };

            if let Some(msg) = msg {
                if let Some(sender) = sender.borrow_mut().take() {
                    sender.send(msg);
                }

                let _ = listener.borrow_mut().take();
            }
        })));

        self.try_post_forget().ok()?;

        receiver.await.ok()
    }

    //Simple - post to iframe and forget about the response
    //Only used for initial setup really
    pub fn try_post_forget(self) -> Result<(), PostError> {
        WALLET_IFRAME.with(|iframe| {
            match iframe.borrow().as_ref() {
                None => Err(JsValue::from_str("Iframe doesn't exist yet!")),
                Some(iframe) => {
                    (self.id, self.data).try_post_to(iframe.content_window().unwrap_ext())
                }
            }
        })
        .map_err(|err| PostError::Iframe(err))
    }
}

//Top-level container messages
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

impl IframeMsgSend for WalletBridgeMsg {}
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

impl IframeMsgSend for WalletBridgeStatus {}
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
    ContractInstantiate(ContractInstantiateMsg),
    ContractExecute(ContractExecuteMsg),
}

impl IframeMsgSend for WalletBridgeRequest {}
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
    ContractInstantiate(Option<String>),
}

impl IframeMsgSend for WalletBridgeResponse {}
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
 
impl IframeMsgSend for WalletBridgeSetup {}
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

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ContractInstantiateMsg {
    pub id: u64
}

impl ContractInstantiateMsg {
    pub async fn instantiate(self) -> Option<String> {
        match WalletBridgeRequest::ContractInstantiate(self).request().await {
            WalletBridgeResponse::ContractInstantiate(addr) => addr,
            _ => None
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ContractExecuteMsg {
    pub addr: String,
    pub msg: shared::execute::ExecuteMsg, 
    pub coins: Option<Coins>
}

/*
impl ContractExecuteMsg {
    pub async fn execute(self) -> Option<String> {
        match WalletBridgeRequest::ContractExecute(self).request().await {
            WalletBridgeResponse::ContractExecute(resp) => resp,
            _ => None
        }
    }
}
*/