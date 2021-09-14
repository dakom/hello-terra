use std::rc::Rc;
use dominator::{html, Dom, clone};
use super::{state::*, styles};
use crate::components::{button::*, image::*, overlay::*};
use crate::utils::prelude::*;
use futures_signals::signal::SignalExt;

impl Registry {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("div", {
            .class(&*styles::PAGE)
            .children(&mut [
                html!("h1", {
                    .text_signal(state.contract_id.signal_ref(|contract_id| {
                        match contract_id {
                            _ => "Welcome!"
                        }
                    }))
                }),
                html!("i", {
                    .text(&format!("wallet address: {}", state.wallet_addr))
                }),
            ])
            .child_signal(state.contract_id.signal_ref(clone!(state => move |contract_id| Some({
                match contract_id {
                    None => {
                        html!("div", {
                            .class(&*styles::CHOICES)
                            .child(html!("div", {
                                .class(&*styles::CHOICE)
                                .child(html!("i", {
                                    .class(&*styles::CHOICE_LABEL)
                                    .text("System contract is outdated or not found")
                                }))
                                .child(Button::new_color(ButtonColor::Blue, "Bootstrap fresh system")
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
                                    .event(clone!(state => move |_evt:events::Change| {
                                        let file =
                                            state.file_input.borrow().as_ref()
                                                .and_then(|input| input.files())
                                                .and_then(|files| files.get(0));

                                        if let Some(file) = file {
                                            Self::upload_contract_file(state.clone(), file);
                                        }
                                    }))
                                }))
                            }))
                        })
                    },
                    Some(contract_id) => {
                        html!("div", {
                            .class(&*styles::CHOICES)
                            .child(Button::new_color(ButtonColor::Blue, "Register new account")
                                .render_mixin(clone!(state => move |dom| {
                                    dom
                                        .event(clone!(state => move |evt:events::Click| {
                                        }))
                                }))
                            )
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
            .child_signal(state.loader.is_loading().map(|is_loading| {
                if is_loading {
                    //Cancelling the terra window leaves it hanging
                    Some(html!("h1", {.text("Waiting...")}))
                    //Some(Overlay::new().render_loader())
                } else {
                    None
                }
            }))
        })
    }
}