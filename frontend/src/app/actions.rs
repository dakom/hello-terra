use std::rc::Rc;
use super::state::App;
use crate::utils::{prelude::*, wallet_bridge::{WalletBridgeMsgWrapper, WalletBridgeSetupRequest, WalletBridgeStatus, WalletBridgeWindowEvent}};
use dominator::clone;
use wasm_bindgen_futures::spawn_local;

impl App {
    pub fn logout(&self) {
        let _ = WalletBridgeSetupRequest::Disconnect.request_forget();
    }

    fn request_wallet_address(state: Rc<Self>) {
        spawn_local(clone!(state => async move {
            match WalletBridgeSetupRequest::WalletInfo.request().await {
                Ok(info) => {
                    state.wallet_info.set(info);
                    state.initializing.set_neq(false);
                },
                _ => {
                    log::info!("error getting wallet info");
                }
            }
        }));
    }

    //App is the only place with this top-level handler
    //and it needs insight into the raw WalletBridge wrappers
    pub fn handle_wallet_message(state: Rc<Self>, msg: WalletBridgeMsgWrapper<()>) {
        match msg {
            WalletBridgeMsgWrapper::Status(status) => {
                log::info!("CHANGING STATUS: {:?}", status);

                match status {
                    WalletBridgeStatus::Initializing | WalletBridgeStatus::Wallet_Not_Connected => {
                        state.initializing.set_neq(status == WalletBridgeStatus::Initializing); 
                        state.wallet_info.set(None);
                        state.contract_info.set(None);
                    },
                    WalletBridgeStatus::Wallet_Connected => {
                        if state.wallet_info.lock_ref().is_none() {
                            state.initializing.set_neq(true);
                            Self::request_wallet_address(state.clone());
                        } else {
                            state.initializing.set_neq(false);
                        }
                    }
                }
            },

            WalletBridgeMsgWrapper::WindowEvent(event) => {
                match event {
                    WalletBridgeWindowEvent::Click => {
                        state.iframe_visible.set_neq(false);
                    }
                }
            },

            _ => { }

        }
    }
}