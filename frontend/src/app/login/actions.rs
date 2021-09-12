use super::state::*;
use std::rc::Rc;
use dominator::clone;
use crate::utils::{prelude::*, terra::TERRA};

impl Login {
    pub fn do_login_key(state: Rc<Self>) {

        state.loader.load(clone!(state => async move {
            if let Some(input_value) = state.input_value.borrow().as_ref() {
                let wallet_id = TERRA.login(input_value);
                state.app.wallet_id.set(Some(wallet_id));
            } else {
                state.input_error.set(Some(LoginError::InvalidWalletId.as_string()));
            }
        }));
    }

    pub fn do_login_extension(state: Rc<Self>) {

        state.loader.load(clone!(state => async move {
        }));
    }
    pub fn clear_input_error(&self) {
        self.input_error.set_neq(None);
    }
}

enum LoginError {
   InvalidWalletId 
}

impl LoginError {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::InvalidWalletId => "Invalid wallet id"
        }
    }

    fn as_string(&self) -> String {
        self.as_str().to_string()
    }
}