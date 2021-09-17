use std::rc::Rc;
use cosmwasm_std::{Addr, Decimal};
use dominator::{html, Dom, clone};
use super::{state::*, styles};
use crate::components::{button::*, image::*};
use futures_signals::signal::SignalExt;
use crate::utils::prelude::*;
use shared::{execute::{ExecuteMsg, AccountSummary}};

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


        html!("h1", {
            .future(clone!(state => async move {
                let summary = ContractExecuteMsg{
                    addr: state.contract_info.addr.clone(),
                    msg: ExecuteMsg::GetAccountSummary,
                    coins: None,
                }.execute::<AccountSummary>().await;

                log::info!("{:?}", summary);
                /*
                let summary = ContractQueryMsg(AccountSummary {
                    name: "foo".to_string(),
                    addr: Addr::unchecked("bar"),
                    total_history: Decimal::one()
                })
                    .query::<AccountSummary>()
                    .await;
                    */
                /*
                let summary = ContractQueryMsg(QueryMsg::AccountSummary)
                    .query::<AccountSummary>()
                    .await;
                    */
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
                            .text("system id:")
                        }),
                        html!("td", {
                            .text(&format!("{}", self.contract_info.code_id))
                        })
                    ])
                })
            ])
        })
    }
}