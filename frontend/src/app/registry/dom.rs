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
                        //Got the hash - first try to see if we have the id and/or full info lookup
                        //if we do have any of these, the UI will reflect that
                        //including if we have the full info - it'll move onto the account page
                        let contract_id = Self::get_contract_id(&ContractIdLookupKey {
                            contract_hash: hash.clone(), 
                            chain_id: state.wallet_info.chain_id.clone()
                        });
                        if let Some(contract_id) = contract_id.as_ref() {
                            let contract_info = Self::get_contract_addr(&ContractAddrLookupKey {
                                chain_id: state.wallet_info.chain_id.clone(),
                                contract_id: contract_id.clone(),
                                wallet_addr: state.wallet_info.addr.clone()
                            })
                            .map(|contract_addr| {
                                ContractInfo {
                                    id: contract_id.clone(),
                                    addr: contract_addr,
                                    chain_id: state.wallet_info.addr.clone()
                                }
                            });

                            state.app.contract_info.set_neq(contract_info);
                        }
                        state.contract_id.set_neq(contract_id);
                        state.contract_hash.set_neq(Some(hash));
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
                            _ => "Welcome!"
                        }
                    })))
                }),
            ])
            .child_signal(state.contract_id.signal_ref(clone!(state => move |contract_id| Some({
                match contract_id {
                    None => {
                        html!("div", {
                            .child(html!("i", {
                                .style("display", "block")
                                .style("text-align", "center")
                                .text(&format!("No system found. Bootstrap a fresh one!"))
                            }))
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
                        })
                    },
                    Some(contract_id) => {
                        html!("div", {
                            .class(&*styles::CHOICES)
                            .child(html!("div", {
                                .class(&*styles::CHOICE)
                                .child(Button::new_color(ButtonColor::Blue, "Register new account")
                                    .render_mixin(clone!(state, contract_id => move |dom| {
                                        dom
                                            .event(clone!(state, contract_id => move |evt:events::Click| {
                                                Self::instantiate_contract(state.clone(), contract_id.clone());
                                            }))
                                    }))
                                )
                            }))
                        })
                    }
                }
            }))))
            .child(
                html!("table", {
                    .class(&*styles::META_INFO)
                    .children(&mut [
                        html!("tr", {
                            .children(&mut [
                                html!("td", {
                                    .text("network:")
                                }),
                                html!("td", {
                                    .text(&format!("{}", state.wallet_info.network_name))
                                })
                            ])
                        }),
                        html!("tr", {
                            .children(&mut [
                                html!("td", {
                                    .text("wallet address:")
                                }),
                                html!("td", {
                                    .text(&format!("{}", state.wallet_info.addr))
                                })
                            ])
                        }),
                        html!("tr", {
                            .children(&mut [
                                html!("td", {
                                    .text("system hash:")
                                }),
                                html!("td", {
                                    .text(&contract_hash)
                                })
                            ])
                        })
                    ])
                    .child_signal(state.contract_id.signal_ref(|contract_id| {
                        contract_id.as_ref().map(|contract_id| {
                            html!("tr", {
                                .children(&mut [
                                    html!("td", {
                                        .text("system id:")
                                    }),
                                    html!("td", {
                                        .text(&format!("{}", contract_id))
                                    })
                                ])
                            })
                        })
                    }))
                })
            )
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