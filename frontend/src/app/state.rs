use std::rc::Rc;
use futures_signals::{map_ref, signal::{Mutable, Signal, SignalExt}};
use crate::{config::DEBUG, utils::terra::TERRA};

pub struct App {
    pub wallet_id: Mutable<Option<String>>,
    pub contract_id: Mutable<Option<String>>,
}

impl App {
    pub fn new() -> Rc<Self> {

        let wallet_id = DEBUG.wallet_mnemonic.as_ref().map(|key| TERRA.login(key));

        Rc::new(Self {
            wallet_id: Mutable::new(wallet_id),
            contract_id: Mutable::new(None),
        })
    }

    pub fn wallet_contract_signal(&self) -> impl Signal<Item = (Option<String>, Option<String>)> {
        map_ref! {
            let wallet_id = self.wallet_id.signal_cloned(),
            let contract_id = self.contract_id.signal_cloned()
                => {
                    (wallet_id.clone(), contract_id.clone())
                }
        }
    } 
}