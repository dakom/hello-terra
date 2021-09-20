use std::{rc::Rc, cell::RefCell};
use dominator_helpers::futures::AsyncLoader;
use web_sys::HtmlInputElement;
use futures::channel::oneshot::Sender;
use crate::app::App;
use futures_signals::signal::Mutable;
use crate::utils::prelude::*;

pub struct Registry {
    pub wallet_info: WalletInfo,
    pub app: Rc<App>,
    pub error: Mutable<Option<String>>
}

impl Registry {
    pub fn new(wallet_info: WalletInfo, app: Rc<App>) -> Rc<Self> {
        Rc::new(Self {
            wallet_info,
            app,
            error: Mutable::new(None),
        })
    }
}