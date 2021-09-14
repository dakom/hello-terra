use std::rc::Rc;
use dominator::{html, Dom, clone, with_node};
use super::{state::*, styles};
use crate::components::{button::*, image::*, overlay::*};
use crate::utils::prelude::*;
use wasm_bindgen::prelude::*;
use futures_signals::signal::SignalExt;

impl Registry {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("div", {
            .future(clone!(state => async move {
                match Self::load_contract_hash().await {
                    Ok(hash) => {
                        let contract_id = Self::get_contract_id(&hash, &state.wallet_info.chain_id);
                        if let Some(contract_id) = contract_id.as_ref() {
                            let contract_info = Self::get_contract_addr(&state.wallet_info.addr, &state.wallet_info.chain_id)
                                .map(|contract_addr| {
                                    ContractInfo {
                                        id: contract_id.clone(),
                                        addr: contract_addr,
                                        chain_id: state.wallet_info.addr.clone()
                                    }
                                });
                            state.app.contract_info.set(contract_info);
                        }
                        state.contract_id.set(contract_id);
                        state.contract_hash.set(Some(hash));
                    },
                    Err(err) => {
                        log::warn!("got error: {:?}", err);
                    }
                }
            }))
            .child_signal(state.contract_hash.signal_ref(clone!(state => move |contract_hash| {
                match contract_hash {
                    None => {
                        Some(html!("div", {
                            .class(&*styles::PAGE)
                            .children(&mut [
                                html!("h1", {
                                    .text("Loading system info...")
                                })
                            ])
                        }))
                    },
                    Some(contract_hash) => {
                        Some(Self::_render(state.clone(), contract_hash.to_string()))
                    }
                }
            })))
        })
    }

    fn _render(state: Rc<Self>, contract_hash: String) -> Dom {
        html!("div", {
            .class(&*styles::PAGE)
            .children(&mut [
                html!("h1", {
                    .text_signal(state.contract_id.signal_ref(clone!(state => move |contract_id| {
                        match contract_id {
                            _ => format!("Welcome to {}!", state.wallet_info.network_name)
                        }
                    })))
                }),
                html!("i", {
                    .text(&format!("wallet address: {}", state.wallet_info.addr))
                }),
            ])
            .child_signal(state.contract_id.signal_ref(clone!(state => move |contract_id| Some({
                match contract_id {
                    None => {
                        html!("div", {
                            .child(html!("div", {
                                .class(&*styles::CHOICES)
                                .child(html!("div", {
                                    .child(Button::new_color(ButtonColor::Blue, "Automatic Builtin")
                                        .render_mixin(clone!(state => move |dom| {
                                            dom
                                                .event(clone!(state => move |evt:events::Click| {
                                                    Self::upload_contract_remote(state.clone());
                                                }))
                                        }))
                                    )
                                }))
                                .child(html!("h1", {
                                    .text("-- OR --")
                                    .style("text-align", "center")
                                    .style("opacity", "0.7")
                                }))
                                .child(html!("div", {
                                    .class(&*styles::CHOICE)
                                    .child(Button::new_color(ButtonColor::Blue, "Direct upload")
                                        .render_mixin(clone!(state => move |dom| {
                                            dom
                                                .event(clone!(state => move |evt:events::Click| {
                                                    if let Some(elem) = state.file_input.borrow().as_ref() {
                                                        elem.click();
                                                    }
                                                }))
                                        }))
                                    )
                                    .child(html!("input" => web_sys::HtmlInputElement, {
                                        .property("type", "file")
                                        .style("display", "none")
                                        .after_inserted(clone!(state => move |elem| {
                                            *state.file_input.borrow_mut() = Some(elem);
                                        }))
                                        .with_node!(elem => {
                                            .event(clone!(state => move |_evt:events::Change| {
                                                let file = elem.files().and_then(|files| files.get(0));

                                                if let Some(file) = file {
                                                    Self::upload_contract_file(state.clone(), file);
                                                }
                                        
                                                // Clear it to enable working again
                                                elem.set_value("");
                                            }))
                                        })
                                    }))
                                }))
                            }))
                            .child(html!("i", {
                                .style("display", "block")
                                .style("text-align", "center")
                                .text(&format!("No system found. Bootstrap a fresh one!"))
                            }))
                        })
                    },
                    Some(contract_id) => {
                        html!("div", {
                            .class(&*styles::CHOICES)
                            .child(html!("div", {
                                .class(&*styles::CHOICE)
                                .child(html!("i", {
                                    .class(&*styles::CHOICE_LABEL)
                                    .text(&format!("System id: {}", contract_id))
                                }))
                                .child(Button::new_color(ButtonColor::Blue, "Register new account")
                                    .render_mixin(clone!(state => move |dom| {
                                        dom
                                            .event(clone!(state => move |evt:events::Click| {
                                            }))
                                    }))
                                )
                            }))
                        })
                    }
                }
            }))))
            .global_event(clone!(state => move |evt:dominator_helpers::events::Message| {
                if let Ok(msg) = evt.try_serde_data::<WalletMsg>() {
                    Self::handle_wallet_message(state.clone(), msg);
                } else {
                    //example: log::info!("{}", WalletMsg::Status("hello".to_string()).to_json_string());
                    log::error!("hmmm got other iframe message...");
                }
            }))
            //Cancelling the terra window leaves it hanging
            .child_signal(state.loader.is_loading().map(|is_loading| {
                if is_loading {
                    Some(html!("h1", {.text("Waiting...")}))
                } else {
                    None
                }
            }))
        })
    }
}