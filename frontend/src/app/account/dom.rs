use std::rc::Rc;
use dominator::{html, Dom, clone};
use super::{state::*, styles};
use crate::components::{button::*, image::*};
use futures_signals::signal::SignalExt;
use crate::utils::prelude::*;

impl Account {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("div", {
            .class(&*styles::PAGE)
            .child_signal(state.funds.signal_cloned().map(clone!(state => move |funds| {
                Some(match funds {
                    Some(funds) => Self::render_funds(state.clone(), funds),
                    None => Self::render_loading(state.clone()), 
                })
            })))
            .child(state.render_footer())
        })
    }

    fn render_funds(state: Rc<Self>, funds: Rc<Funds>) -> Dom {
        html!("h1", {
            .text("Funds Manager")
        })
    }
    fn render_loading(state: Rc<Self>) -> Dom {

        shared::summary::FullSummaryRequest {}
            .post(&state.wallet_info.addr, None);

        html!("h1", {
            .text("Loading...")
            .global_event(clone!(state => move |evt:dominator_helpers::events::Message| {
                if let Ok(msg) = evt.try_serde_data::<WalletMsg>() {
                    Self::handle_loading_message(state.clone(), msg);
                } else {
                    //example: log::info!("{}", WalletMsg::Status("hello".to_string()).to_json_string());
                    log::error!("hmmm got other iframe message...");
                }
            }))
        })
    }

    fn render_footer(&self) -> Dom {
        html!("table", {
            .class(&*styles::META_INFO)
            .children(&mut [
                html!("tr", {
                    .children(&mut [
                        html!("td", {
                            .text("network:")
                        }),
                        html!("td", {
                            .text(&format!("{}", self.wallet_info.network_name))
                        })
                    ])
                }),
                html!("tr", {
                    .children(&mut [
                        html!("td", {
                            .text("wallet address:")
                        }),
                        html!("td", {
                            .text(&format!("{}", self.wallet_info.addr))
                        })
                    ])
                }),
                html!("tr", {
                    .children(&mut [
                        html!("td", {
                            .text("system id:")
                        }),
                        html!("td", {
                            .text(&format!("{}", self.contract_info.id))
                        })
                    ])
                })
            ])
        })
    }
}