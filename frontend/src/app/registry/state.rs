use std::{rc::Rc, cell::RefCell};
use dominator_helpers::futures::AsyncLoader;
use web_sys::HtmlInputElement;
use futures::channel::oneshot::Sender;
use crate::app::App;
use futures_signals::signal::Mutable;

pub struct Registry {
    pub wallet_addr: String,
    pub app: Rc<App>,
    pub file_input: RefCell<Option<HtmlInputElement>>,
    pub loader: AsyncLoader,
    pub contract_id: Mutable<Option<String>>,
    pub msg_sender: RefCell<Option<Sender<Option<String>>>>
}

impl Registry {
    pub fn new(wallet_addr: String, app: Rc<App>) -> Rc<Self> {

        log::warn!("TODO - load contract Hash->ID from LocalStorage");

        Rc::new(Self {
            wallet_addr,
            contract_id: Mutable::new(None),
            app,
            file_input: RefCell::new(None),
            loader: AsyncLoader::new(),
            msg_sender: RefCell::new(None),
        })
    }
}