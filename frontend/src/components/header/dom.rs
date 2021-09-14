use std::rc::Rc;
use dominator::{html, Dom, clone};
use super::{state::*, styles};
use crate::components::{button::*, image::*};
use crate::utils::prelude::*;
use futures_signals::signal::SignalExt;

impl Header {
    pub fn render(state: Rc<Self>) -> Dom {
        html!("nav", {
            .class(&*styles::NAV)
            .child(
                Image::new(ImageKind::TerraLogo)
                    .render_mixin(|dom| {
                        dom.attribute("height", styles::LOGO_HEIGHT)
                    })
            )
            .child(html!("div", {
                .class(&*styles::TOP_RIGHT)
                .children_signal_vec(
                    state.app.wallet_addr.signal_ref(clone!(state => move |wallet_addr| {
                        let mut children:Vec<Dom> = Vec::new();
                        if wallet_addr.is_some() {
                            children.push(
                                Button::new_color(ButtonColor::Blue, "Disconnect")
                                    .render_mixin(clone!(state => move |dom| {
                                        dom.event(clone!(state => move |evt:events::Click| {
                                            state.app.logout();
                                        }))
                                    }))
                            );
                        } 

                        children
                    }))
                    .to_signal_vec()
                )
            }))
        })
    }
}