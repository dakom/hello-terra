use std::rc::Rc;
use super::state::App;
use crate::utils::prelude::*;

impl App {
    pub fn logout(&self) {
        WalletMsg::Setup(WalletSetup::Disconnect).post();
    }

    pub fn handle_wallet_message(state: Rc<Self>, msg: WalletMsg) {
        match msg {
            WalletMsg::Status(status) => {
                match status {
                    WalletStatus::Initializing | WalletStatus::Wallet_Not_Connected => {
                        state.initializing.set_neq(status == WalletStatus::Initializing); 
                        state.wallet_addr.set_neq(None);
                        state.contract_addr.set_neq(None);
                    },
                    WalletStatus::Wallet_Connected => {
                        if state.wallet_addr.lock_ref().is_none() {
                            state.initializing.set_neq(true);
                            WalletMsg::Request(WalletRequest::Addr).post();
                        } else {
                            state.initializing.set_neq(false);
                        }
                    }
                }
            },

            WalletMsg::Window(event) => {
                match event {
                    WalletWindowEvent::Click => {
                        state.iframe_visible.set_neq(false);
                    }
                }
            },
            WalletMsg::Response(resp) => {
                match resp {
                    WalletResponse::Addr(addr) => {
                        match addr {
                            Some(addr) => {
                                state.wallet_addr.set(Some(addr));
                                state.initializing.set_neq(false);
                            },
                            None => {
                                state.logout();
                            }
                        }
                    },
                    //APP is not interested in other responses
                    _ => {
                    }
                    /*
                    WalletResponse::ContractUpload(id) => {
                        match id {
                            Some(id) => {
                                log::info!("Got contract id: {}", id);
                            },
                            None => {
                                log::info!("unable to upload contract!");
                            }
                        }
                    }
                    */
                }
            },

            WalletMsg::Request(_) => {
                log::warn!("Strange, got wallet request on parent frame...");
            },
            WalletMsg::Setup(_) => {
                log::warn!("Strange, got wallet setup on parent frame...");
            }
        }
    }
}