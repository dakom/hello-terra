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
    pub contract_hash: Mutable<Option<String>>,
    pub file_input: RefCell<Option<HtmlInputElement>>,
    pub loader: AsyncLoader,
    pub contract_id: Mutable<Option<u64>>,
    pub contract_id_sender: RefCell<Option<Sender<Option<u64>>>>,
    pub contract_addr_sender: RefCell<Option<Sender<Option<String>>>>,
}

impl Registry {
    pub fn new(wallet_info: WalletInfo, app: Rc<App>) -> Rc<Self> {
        Rc::new(Self {
            wallet_info,
            contract_hash: Mutable::new(None),
            contract_id: Mutable::new(None),
            contract_id_sender: RefCell::new(None),
            contract_addr_sender: RefCell::new(None),
            app,
            file_input: RefCell::new(None),
            loader: AsyncLoader::new(),
        })
    }
}