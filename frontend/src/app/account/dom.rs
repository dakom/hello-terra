use std::rc::Rc;
use std::collections::HashSet;
use cosmwasm_std::{Addr, Decimal};
use dominator::{html, Dom, clone, with_node};
use super::{state::*, styles, manager::Manager};
use crate::{components::{button::*, image::*}, utils::wallet_bridge::ContractQueryMsg};
use futures_signals::signal::SignalExt;
use crate::utils::prelude::*;
use shared::{
    coin::CoinDenom, 
    contracts::{
        hub::{
            query as hub_query,
            execute as hub_execute
        },
        account::{
            query as account_query,
            execute as account_execute
        }
    }
};

impl Account {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("div", {
            .class(&*styles::PAGE)
            .child_signal(state.available_coins.signal_cloned().map(clone!(state => move |available_coins| {
                Some(match available_coins {
                    Some(available_coins) => Self::render_available_coins(state.clone(), available_coins),
                    None => Self::render_loading(state.clone()), 
                })
            })))
            .child(state.render_footer())
        })
    }

    fn render_available_coins(state: Rc<Self>, available_coins: Vec<CoinDenom>) -> Dom {
        html!("div", {
            .class(&*styles::PAGE)
            .child(html!("h1", { .text("Funds Manager") }))
            .child({
                if available_coins.len() > 0 {
                    html!("select" => web_sys::HtmlSelectElement, {
                        .style("font-size", "18rem")
                        .children(available_coins.iter().map(|coin_denom| {
                            html!("option", {
                                .attribute("value", coin_denom)
                                .text(coin_denom)
                                .property_signal("selected", state.selected_coin.signal_ref(clone!(coin_denom => move |selected_coin| {
                                    selected_coin.as_ref() == Some(&coin_denom)
                                })))
                            })
                        }))
                        .with_node!(elem => {
                            .event(clone!(state => move |evt:events::Change| {
                                state.selected_coin.set(Some(elem.value()));
                            }))
                        })
                    })
                } else {
                    html!("h1", { .text("No known coins!") })
                }
            })
            .child_signal(state.selected_coin.signal_cloned().map(clone!(state => move |coin| {
                coin.map(|coin| {
                    Manager::render(Manager::new(state.clone(), coin))
                })
            })))
        })
    }

    fn render_loading(state: Rc<Self>) -> Dom {
        html!("h1", {
            .future(clone!(state => async move {
                let coins_deposited_ever = ContractQueryMsg {
                    addr: state.contract_info.hub_addr.clone(),
                    msg: hub_query::QueryMsg::AvailableCoins
                }.query::<hub_query::AvailableCoins>().await.unwrap_ext();

                let coins_in_wallet = ContractQueryMsg {
                    addr: state.contract_info.account_addr.clone(),
                    msg: account_query::QueryMsg::AvailableCoinsInWallet
                }.query::<account_query::AvailableCoinsInWallet>().await.unwrap_ext();

                let mut all_coins:HashSet<CoinDenom> = HashSet::new();

                for coin_denom in coins_in_wallet.list {
                    all_coins.insert(coin_denom);
                }

                for coin_denom in coins_deposited_ever.list {
                    all_coins.insert(coin_denom);
                }

                /*all_coins.insert("FOO".to_string());
                all_coins.insert("BAR".to_string());
                all_coins.insert("UsT".to_string());
                */

                if let Some(value) = all_coins.iter().find(|coin| {
                    coin.to_lowercase() == "ust"
                }) {
                    state.selected_coin.set(Some(value.clone()));
                } else if let Some(value) = all_coins.iter().next() { 
                    state.selected_coin.set(Some(value.clone()));
                }


                state.available_coins.set(Some(all_coins.into_iter().collect()))
            }))
            .text("Loading Funds...")
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
                            .text("hub address:")
                        }),
                        html!("td", {
                            .text(&format!("{}", self.contract_info.hub_addr))
                        })
                    ])
                }),
                html!("tr", {
                    .children(&mut [
                        html!("td", {
                            .text("account address:")
                        }),
                        html!("td", {
                            .text(&format!("{}", self.contract_info.account_addr))
                        })
                    ])
                })
            ])
        })
    }
}