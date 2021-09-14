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
}
