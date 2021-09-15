use super::state::*;
use std::rc::Rc;
use dominator::clone;
use crate::utils::prelude::*;

impl Login {
    pub fn do_login_extension(state: Rc<Self>) {
        WalletMsg::Setup(WalletSetup::ConnectExtension).post();
    }
    pub fn do_login_mobile(state: Rc<Self>) {
        state.app.iframe_visible.set_neq(true);
        WalletMsg::Setup(WalletSetup::ConnectMobile).post();
    }
    pub fn do_login_manually(state: Rc<Self>) {
        match (
            state.input_key_value.borrow().as_ref(),
            state.input_host_value.borrow().as_ref(),
            state.input_chain_value.borrow().as_ref(),
        ) {
            (Some(key), Some(host), Some(chain)) => {
                WalletMsg::Setup(WalletSetup::ConnectManually(key.to_string(), host.to_string(), chain.to_string())).post();
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
