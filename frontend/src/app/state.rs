use std::rc::Rc;
use futures_signals::{map_ref, signal::{Mutable, Signal, SignalExt}};
use crate::{config::DEBUG, utils::terra::TERRA};

pub struct App {
    pub initializing: Mutable<bool>,
    pub wallet_id: Mutable<Option<String>>,
    pub contract_id: Mutable<Option<String>>,
    pub iframe_visible: Mutable<bool>,
}

impl App {
    pub fn new() -> Rc<Self> {

        let wallet_id = DEBUG.wallet_mnemonic.as_ref().map(|key| TERRA.login(key));

        Rc::new(Self {
            initializing: Mutable::new(true),
            wallet_id: Mutable::new(wallet_id),
            contract_id: Mutable::new(None),
            iframe_visible: Mutable::new(false),
        })
    }

    pub fn wallet_contract_init_signal(&self) -> impl Signal<Item = (bool, Option<String>, Option<String>)> {
        map_ref! {
            let is_initializing = self.initializing.signal_cloned(),
            let wallet_id = self.wallet_id.signal_cloned(),
            let contract_id = self.contract_id.signal_cloned()
                => {
                    (*is_initializing, wallet_id.clone(), contract_id.clone())
                }
        }
    } 
}