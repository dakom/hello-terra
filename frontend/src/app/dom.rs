use dominator::{html, Dom, clone};
use std::rc::Rc;
use futures_signals::signal::SignalExt;
use crate::{
    components::{
        overlay::Overlay,
        header::Header,
    }, 
    config::REMOTE_TARGET, 
    utils::{
        prelude::*,
        wallet_bridge::{WALLET_IFRAME, WalletBridgeMsgWrapper}
    }
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
            .child_signal(state.wallet_contract_init_signal().map(clone!(state => move |(is_initializing, wallet_info, hub_contract_info)| {
                log::info!("INITIALIZING: {}", is_initializing);
                if is_initializing {
                    Some(Overlay::new().render_loader())
                } else {
                    match (wallet_info, hub_contract_info) {
                        (None, _) => Some(Login::render(Login::new(state.clone()))),
                        (Some(wallet_info), None) => Some(Registry::render(Registry::new(wallet_info, state.clone()))),
                        (Some(wallet_info), Some(contract_info)) => Some(Account::render(Account::new(wallet_info, contract_info, state.clone()))),
                        _ => panic!("can't have a contract without a wallet!")
                    }
                }
            })))
            .child(html!("iframe" => web_sys::HtmlIFrameElement, {
                .attribute("src", REMOTE_TARGET.iframe_url())
                .after_inserted(|elem| {
                    WALLET_IFRAME.with(|iframe| *iframe.borrow_mut() = Some(elem));
                })
                //this is kinda funny... hide/show the iframe if we need the QR Modal
                .class_signal(styles::IFRAME_HIDDEN.clone(), state.iframe_visible.signal().map(|x| !x))
                .class_signal(styles::IFRAME_VISIBLE.clone(), state.iframe_visible.signal())
            }))
            //WalletStatus is sent from the iframe at any time - not part of a request/response
            //WindowClick is also simpler to handle globally
            //So App needs this top-level listener to iframe messages
            //But it is the *only* place. Everywhere else is proper Futures
            .global_event(clone!(state => move |evt:dominator_helpers::events::Message| {
                //log::info!("EXAMPLE: {}", serde_json::to_string(&crate::utils::wallet_bridge::WalletBridgeMsgWrapper::Status(crate::utils::wallet_bridge::WalletBridgeStatus::Wallet_Not_Connected)).unwrap_ext());
                match evt.try_serde_data::<(u64, String, Result<WalletBridgeMsgWrapper<()>, String>)>() {
                    Ok((bridge_id, tag, msg)) => {
                        if tag != crate::utils::wallet_bridge::TAG {
                            log::info!("Not meant for us...");
                        } else {
                            if let Ok(msg) = msg {
                                Self::handle_wallet_message(state.clone(), msg);
                            }
                        }
                    },
                    Err(err) => {
                        //log::info!("serialization error: {:?}", err);
                    }
                } 
            }))
        })
    }
}