use std::rc::Rc;
use dominator::{html, Dom, clone, with_node};
use super::{state::*, styles};
use crate::components::{button::*, image::*, overlay::*};
use crate::utils::{prelude::*, contract::*};
use wasm_bindgen::prelude::*;
use futures_signals::signal::SignalExt;
use crate::config::DEBUG;
use crate::utils::contract;
use futures::join;

impl Registry {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("div", {
            .future(state.phase.signal_cloned().for_each(clone!(state => move |phase| {
                clone!(state, phase => async move {
                    match phase {
                        Phase::Checking => {
                            match contract::check_requires_bootstrap(&state.wallet_info).await {
                                Ok(flag) => {
                                    if(flag) {
                                        state.phase.set(Phase::WaitUser)
                                    } else {
                                        state.phase.set(Phase::Bootstrapping)
                                    }
                                },
                                Err(err) => {
                                    state.phase.set(Phase::Error("storage error".to_string()));
                                }
                            }
                        },
                        Phase::Bootstrapping => {
                            match contract::bootstrap_registry(&state.wallet_info).await {
                                Ok(info) => {
                                    state.app.contract_info.set(Some(info));
                                },
                                Err(err) => {
                                    state.phase.set(Phase::Error(err.to_string()));
                                }

                            }
                        },
                        _ => {}
                    }
                })
            })))
            .class(&*styles::PAGE)
            .children(&mut [
                html!("h1", {
                    .text_signal(state.phase.signal_cloned().map(|phase| {
                        match phase {
                            Phase::Error(err) => err,
                            Phase::Checking | Phase::Bootstrapping => "Please wait...".to_string(),
                            Phase::WaitUser => "Welcome!".to_string()
                        }
                    }))
                }),
            ])
            .child_signal(state.phase.signal_cloned().map(clone!(state => move |phase| {
                match phase {
                    Phase::WaitUser => {

                        Some(
                            html!("div", {
                                .class(&*styles::PAGE)
                                .children(&mut [ 
                                    Button::new_color(ButtonColor::Blue, "Bootstrap a Fresh System")
                                        .render_mixin(clone!(state => move |dom| {
                                            dom
                                                .style("margin-top", "10rem")
                                                .event(clone!(state => move |evt:events::Click| {
                                                    state.phase.set(Phase::Bootstrapping);
                                                }))
                                        })),
                                    html!("p", {
                                        .class(&*styles::PAGE)
                                        .style("text-align", "center")
                                        .child(html!("i", { .text("Looks like you're on a brand-new network!") }))
                                        .child(html!("i", { .text("This initial setup may take a minute or two to get up and running") }))
                                        .child(html!("i", { .text("After that, it'll just be the regular login flow") }))
                                    })
                                ])
                            })
                        )
                    },
                    _ => None
                }
            })))
        })
    }
}