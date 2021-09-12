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
                    .text("Welcome!")
                }),
                html!("i", {
                    .text(&format!("wallet id: {}", state.wallet_id))
                }),
                html!("div", {
                    .class(&*styles::CHOICES)
                    .children(&mut [
                        Button::new_color(ButtonColor::Blue, "Upload Contract")
                            .render_mixin(clone!(state => move |dom| {
                                dom
                                    .event(clone!(state => move |evt:events::Click| {
                                        if let Some(elem) = state.file_input.borrow().as_ref() {
                                            elem.click();
                                        }
                                    }))
                            })),
                        html!("h1", {
                            .text("-- OR --")
                            .style("opacity", "0.7")
                        }),
                        Button::new_color(ButtonColor::Blue, "Manage Funds")
                            .render_mixin(clone!(state => move |dom| {
                                dom
                                    .event(clone!(state => move |evt:events::Click| {
                                        //Self::do_login(state.clone());
                                    }))
                            }))
                    ])
                }),
                html!("input" => web_sys::HtmlInputElement, {
                    .property("type", "file")
                    //.property("accept", "image/*")
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
                })
            ])
            .child_signal(state.loader.is_loading().map(|is_loading| {
                if is_loading {
                    Some(Overlay::new().render_loader())
                } else {
                    None
                }
            }))
        })
    }
}