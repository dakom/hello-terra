use std::{rc::Rc, cell::RefCell};
use crate::{app::App, utils::env::env_var};
use futures_signals::signal::Mutable;
use dominator_helpers::futures::AsyncLoader;
use crate::config::{DEFAULT_LOCAL_TERRA_CHAIN, DEFAULT_LOCAL_TERRA_HOST};

pub struct Login {
    pub app: Rc<App>,
    pub input_error: Mutable<Option<String>>,
    pub input_key_value: RefCell<Option<String>>,
    pub input_host_value: RefCell<Option<String>>,
    pub input_chain_value: RefCell<Option<String>>,
}

impl Login {
    pub fn new(app: Rc<App>) -> Rc<Self> {
        let key = env_var("WALLET_MNEMONIC_KEY").ok();
        let host:Option<String> = env_var("WALLET_MNEMONIC_HOST").ok().or(Some(DEFAULT_LOCAL_TERRA_HOST.to_string()));
        let chain:Option<String> = env_var("WALLET_MNEMONIC_CHAIN").ok().or(Some(DEFAULT_LOCAL_TERRA_CHAIN.to_string()));
        Rc::new(Self {
            app,
            input_error: Mutable::new(None),
            input_key_value: RefCell::new(key),
            input_host_value: RefCell::new(host),
            input_chain_value: RefCell::new(chain),
        })
    }
}