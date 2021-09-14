use std::rc::Rc;
use futures_signals::{map_ref, signal::{Mutable, Signal, SignalExt}};
use crate::config::DEBUG;

pub struct App {
    pub initializing: Mutable<bool>,
    pub wallet_addr: Mutable<Option<String>>,
    pub contract_addr: Mutable<Option<String>>,
    pub iframe_visible: Mutable<bool>,
}

impl App {
    pub fn new() -> Rc<Self> {


        Rc::new(Self {
            initializing: Mutable::new(true),
            wallet_addr: Mutable::new(None),
            contract_addr: Mutable::new(None),
            iframe_visible: Mutable::new(false),
        })
    }

    pub fn wallet_contract_init_signal(&self) -> impl Signal<Item = (bool, Option<String>, Option<String>)> {
        map_ref! {
            let is_initializing = self.initializing.signal_cloned(),
            let wallet_addr = self.wallet_addr.signal_cloned(),
            let contract_addr = self.contract_addr.signal_cloned()
                => {
                    (*is_initializing, wallet_addr.clone(), contract_addr.clone())
                }
        }
    } 
}