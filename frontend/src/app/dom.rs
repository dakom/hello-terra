use dominator::{html, Dom, clone};
use std::rc::Rc;
use futures_signals::signal::SignalExt;
use crate::{
    components::{
        overlay::Overlay,
        header::Header,
    },
    utils::prelude::*
};

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
            .child_signal(state.wallet_contract_init_signal().map(clone!(state => move |(is_initializing, wallet_id, contract_id)| {
                if is_initializing {
                    Some(Overlay::new().render_loader())
                } else {
                    match (wallet_id, contract_id) {
                        (None, _) => Some(Login::render(Login::new(state.clone()))),
                        (Some(wallet_id), None) => Some(Registry::render(Registry::new(wallet_id, state.clone()))),
                        (Some(wallet_id), Some(contract_id)) => Some(Account::render(Account::new(wallet_id, contract_id, state.clone()))),
                        _ => panic!("can't have a contract without a wallet!")
                    }
                }
            })))
            .child(html!("iframe" => web_sys::HtmlIFrameElement, {
                .attribute("src", "http://localhost:3000/")
                .after_inserted(|elem| {
                    WALLET_IFRAME.with(|iframe| *iframe.borrow_mut() = Some(elem));
                })
                //this is kinda funny... hide/show the iframe if we need the QR Modal
                .class_signal(styles::IFRAME_HIDDEN.clone(), state.iframe_visible.signal().map(|x| !x))
                .class_signal(styles::IFRAME_VISIBLE.clone(), state.iframe_visible.signal())
            }))
            .global_event(clone!(state => move |evt:dominator_helpers::events::Message| {
                if let Ok(msg) = evt.try_serde_data::<WalletMsg>() {
                    Self::handle_wallet_message(state.clone(), msg);
                } else {
                    //example: log::info!("{}", WalletMsg::Status("hello".to_string()).to_json_string());
                    log::error!("hmmm got other iframe message...");
                }
            }))
        })
    }
}