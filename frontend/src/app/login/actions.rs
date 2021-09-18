use super::state::*;
use std::rc::Rc;
use dominator::clone;
use crate::utils::{prelude::*, wallet_bridge::{WalletBridgeSetupRequest}};

impl Login {
    //Login isn't typical request/response since the WalletState is determined by the provider
    //So just send iframe events, and the status will be reacted to at the top-level app listener
    pub fn do_login_extension(state: Rc<Self>) {
        let _ = WalletBridgeSetupRequest::ConnectExtension.request_forget();
    }
    pub fn do_login_mobile(state: Rc<Self>) {
        state.app.iframe_visible.set_neq(true);
        let _ = WalletBridgeSetupRequest::ConnectMobile.request_forget();
    }
    pub fn do_login_manually(state: Rc<Self>) {
        match (
            state.input_key_value.borrow().as_ref(),
            state.input_host_value.borrow().as_ref(),
            state.input_chain_value.borrow().as_ref(),
        ) {
            (Some(key), Some(host), Some(chain)) => {
                let _ = WalletBridgeSetupRequest::ConnectManually(key.to_string(), host.to_string(), chain.to_string()).request_forget();
            }
            (None, _, _) => {
                state.input_error.set(Some("Enter a key".to_string()));
            },
            (_, None, _) => {
                state.input_error.set(Some("Enter a network".to_string()));
            }
            (_, _, None) => {
                state.input_error.set(Some("Enter a chain id".to_string()));
            }
        }
    }

    pub fn clear_input_error(&self) {
        self.input_error.set_neq(None);
    }
}
