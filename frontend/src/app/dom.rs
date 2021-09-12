use dominator::{html, Dom, clone};
use std::rc::Rc;
use futures_signals::signal::SignalExt;
use crate::components::header::Header;
use super::{
    state::*, 
    styles,
    login::*,
    registry::*,
    account::*,
};

impl App {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("main", {
            .class(&*styles::MAIN) 
            .child(Header::render(Header::new(state.clone())))
            .child_signal(state.wallet_contract_signal().map(clone!(state => move |(wallet_id, contract_id)| {
                match (wallet_id, contract_id) {
                    (None, _) => Some(Login::render(Login::new(state.clone()))),
                    (Some(wallet_id), None) => Some(Registry::render(Registry::new(wallet_id, state.clone()))),
                    (Some(wallet_id), Some(contract_id)) => Some(Account::render(Account::new(wallet_id, contract_id, state.clone()))),
                    _ => panic!("can't have a contract without a wallet!")
                }
            })))
        })
    }
}