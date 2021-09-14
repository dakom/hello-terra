use std::rc::Rc;
use dominator::{html, Dom, clone, with_node};
use futures_signals::signal::SignalExt;
use super::{state::*, styles};
use crate::{
    config::DEBUG,
    utils::prelude::*,
    components::{
        overlay::*,
        button::*, 
        image::*, 
        input::{InputMixin, wrapper::InputWrapper},
    }
};

impl Login {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("div", {
            .class(&*styles::PAGE)
            .child(html!("h1", {
                .text("Login with Terra Station")
            }))
            .child(html!("div", {
                .class(&*styles::CHOICES)
                .children(&mut [
                    Button::new_color(ButtonColor::Red, "")
                        .render_mixin(clone!(state => move |dom| {
                            dom
                                .style("margin-top", "10rem")
                                .event(clone!(state => move |evt:events::Click| {
                                    Self::do_login_extension(state.clone());
                                }))
                                .child(html!("div", {
                                    .class(&*styles::BUTTON_MULTI_LINE)
                                    .children(&mut [
                                        html!("div", {.text("Chrome Browser") }),
                                        html!("div", {.text("Extension") }),
                                    ])
                                }))
                        })),

                    html!("h1", {
                        .text("-- OR --")
                        .style("text-align", "center")
                        .style("opacity", "0.7")
                    }),
                    Button::new_color(ButtonColor::Red, "")
                        .render_mixin(clone!(state => move |dom| {
                            dom
                                .style("margin-top", "10rem")
                                .event(clone!(state => move |evt:events::Click| {
                                    Self::do_login_mobile(state.clone());
                                }))
                                .child(html!("div", {
                                    .class(&*styles::BUTTON_MULTI_LINE)
                                    .children(&mut [
                                        html!("div", {.text("Mobile") }),
                                        html!("div", {.text("Qr Code") }),
                                    ])
                                }))
                        })),
                    /*
                    html!("div", {
                        .class(&*styles::MNEMONIC)
                        .children(&mut [
                            html!("h1", {
                                .text("Mnemonic Key")
                            }),
                            {
                                let mut wrapper = InputWrapper::new();
                                wrapper.error = state.input_error.clone();
                                wrapper.render(
                                    html!("input" => web_sys::HtmlInputElement, {
                                        .apply(InputMixin::text(Some("Enter your wallet mnemonic")))
                                        .apply_if(state.input_value.borrow().is_some(), |dom| {
                                            dom.attribute("value", state.input_value.borrow().as_ref().unwrap_ext())
                                        })
                                        .with_node!(elem => {
                                            .event(clone!(state => move |evt:events::Input| {
                                                let value = evt.value().and_then(|value| {
                                                    if value.is_empty() {
                                                        None
                                                    } else {
                                                        Some(value)
                                                    }
                                                });
                                                *state.input_value.borrow_mut() = value;
                                                state.clear_input_error();
                                            }))
                                        })
                                    })
                                )
                            },
                            Button::new_color(ButtonColor::Blue, "Login")
                                .render_mixin(clone!(state => move |dom| {
                                    dom
                                        .style("margin-top", "10rem")
                                        .event(clone!(state => move |evt:events::Click| {
                                            Self::do_login_key(state.clone());
                                        }))
                                }))
                        ])
                    }),
                    */

                ])
            }))
        })
    }
}