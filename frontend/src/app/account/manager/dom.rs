use std::rc::Rc;
use std::collections::HashSet;
use cosmwasm_std::{Addr, Decimal};
use dominator::{html, Dom, clone, with_node};
use super::{state::*, styles};
use crate::{components::{button::*,input::{InputMixin, wrapper::InputWrapper}, image::*, overlay::Overlay}, utils::wallet_bridge::ContractQueryMsg};
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

impl Manager {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("div", {
            .future(clone!(state => async move {
                state.reload().await;
            }))
            .class(&*styles::PAGE)
            .child_signal(state.funds_signal().map(clone!(state => move |funds| {
                Some(match funds {
                    Some(funds) => {
                        match funds {
                            Ok(funds) => Self::render_funds(state.clone(), funds),
                            Err(err) => html!("h2", {.text(&err) })
                        }
                    }
                    None => html!("h2", {.text("loading...")})
                })
            })))
        })
    }

    fn render_funds(state: Rc<Self>, funds:Rc<Funds>) -> Dom {
        html!("div", {
            .child(
                html!("table", {
                    .children(&mut [
                        html!("tr", {
                            .children(&mut [
                                html!("td", {
                                    .text("account balance:")
                                }),
                                html!("td", {
                                    .text(&format!("{}", funds.account_balance))
                                })
                            ])
                        }),
                        html!("tr", {
                            .children(&mut [
                                html!("td", {
                                    .text("wallet balance:")
                                }),
                                html!("td", {
                                    .text(&format!("{}", funds.wallet_balance))
                                })
                            ])
                        }),
                        html!("tr", {
                            .children(&mut [
                                html!("td", {
                                    .text("account deposit total:")
                                }),
                                html!("td", {
                                    .text(&format!("{}", funds.account_deposit_total))
                                })
                            ])
                        }),
                        html!("tr", {
                            .children(&mut [
                                html!("td", {
                                    .text("global deposit total:")
                                }),
                                html!("td", {
                                    .text(&format!("{}", funds.global_deposit_total))
                                })
                            ])
                        }),
                        html!("tr", { .children(&mut [ html!("td"), html!("td") ]) }),
                        html!("tr", { .children(&mut [ html!("td"), html!("td") ]) }),
                        html!("tr", {
                            .children(&mut [
                                html!("td", {
                                    .child(InputWrapper::new().render(
                                        Self::render_input(funds.clone(), InputKind::Deposit)
                                    ))
                                }),
                                html!("td", {
                                    .child(
                                        Button::new_color(ButtonColor::Red, "Deposit")
                                            .render_mixin(clone!(state, funds => move |dom| {
                                                dom
                                                    .event(clone!(state, funds => move |evt:events::Click| {
                                                        Self::do_deposit(state.clone(), funds.clone());
                                                    }))
                                            }))
                                        )
                                })
                            ])
                        }),
                        html!("tr", { .children(&mut [ html!("td"), html!("td") ]) }),
                        html!("tr", { .children(&mut [ html!("td"), html!("td") ]) }),
                        html!("tr", {
                            .children(&mut [
                                html!("td", {
                                    .child(InputWrapper::new().render(
                                        Self::render_input(funds.clone(), InputKind::Withdrawal)
                                    ))
                                }),
                                html!("td", {
                                    .child(
                                        Button::new_color(ButtonColor::Red, "Withdrawal")
                                            .render_mixin(clone!(state, funds => move |dom| {
                                                dom
                                                    .event(clone!(state, funds => move |evt:events::Click| {
                                                        Self::do_withdrawal(state.clone(), funds.clone());
                                                    }))
                                            }))
                                        )
                                })
                            ])
                        })
                    ])
                })
            )
            .child_signal(funds.input_error.signal_cloned().map(|err| {
                err.map(|err| {
                    html!("h2", {
                        .style("color", crate::config::THEME.error_color)
                        .text(&err)
                    })
                })
            }))
        })
    }


    fn render_input(funds:Rc<Funds>, kind:InputKind) -> Dom {
        let placeholder = match kind {
            InputKind::Deposit => "Deposit Amount",
            InputKind::Withdrawal => "Withdrawal Amount",
        };

        let input_value = match kind {
            InputKind::Deposit => &funds.input_deposit_value, 
            InputKind::Withdrawal => &funds.input_withdrawal_value, 
        };

        html!("input" => web_sys::HtmlInputElement, {
            .apply(InputMixin::text(Some(placeholder)))
            .apply_if(input_value.borrow().is_some(), |dom| {
                dom.attribute("value", input_value.borrow().as_ref().unwrap_ext())
            })
            .with_node!(elem => {
                .event(clone!(funds=> move |evt:events::Input| {
                    let value = evt.value().and_then(|value| {
                        if value.is_empty() {
                            None
                        } else {
                            Some(value)
                        }
                    });

                    let input_value = match kind {
                        InputKind::Deposit => &funds.input_deposit_value, 
                        InputKind::Withdrawal => &funds.input_withdrawal_value, 
                    };

                    *input_value.borrow_mut() = value;

                }))
            })
        })
    }
}


enum InputKind {
    Deposit,
    Withdrawal,
}