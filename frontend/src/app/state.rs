use std::rc::Rc;
use futures_signals::{map_ref, signal::{Mutable, Signal, SignalExt}};
use crate::config::DEBUG;
use crate::utils::prelude::*;

pub struct App {
    pub initializing: Mutable<bool>,
    pub wallet_info: Mutable<Option<WalletInfo>>,
    pub contract_info: Mutable<Option<ContractInfo>>,
    pub iframe_visible: Mutable<bool>,
}

impl App {
    pub fn new() -> Rc<Self> {


        Rc::new(Self {
            initializing: Mutable::new(true),
            wallet_info: Mutable::new(None),
            contract_info: Mutable::new(None),
            iframe_visible: Mutable::new(false),
        })
    }

    pub fn wallet_contract_init_signal(&self) -> impl Signal<Item = (bool, Option<WalletInfo>, Option<ContractInfo>)> {
        map_ref! {
            let is_initializing = self.initializing.signal_cloned(),
            let wallet_info = self.wallet_info.signal_cloned(),
            let contract_info = self.contract_info.signal_cloned()
                => {
                    (*is_initializing, wallet_info.clone(), contract_info.clone())
                }
        }
    } 
}