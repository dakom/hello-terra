use crate::utils::prelude::*;
use std::rc::Rc;
use super::{state::*, styles};
use dominator::{Dom, html, clone};
use futures_signals::signal::SignalExt;

impl InputWrapper {
    pub fn render(&self, child:Dom) -> Dom {
        html!("div", {
            .child(
                html!("div", {
                    .class(&*styles::WRAPPER)
                    .child_signal(self.label.signal_cloned().map(|label| {
                        label.as_ref().map(|label| {
                            html!("label", {
                                .class(&*styles::LABEL)
                                .text(label)
                            })
                        })
                    }))
                    .child(child)
                })
            )
            .child_signal(self.error.signal_cloned().map(|error| {
                error.as_ref().map(|error| {
                    html!("div", {
                        .class(&*styles::ERROR)
                        .text(error)
                    })
                })
            }))
        })
    }
}
