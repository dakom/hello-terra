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
            .future(clone!(state => async move {
                match contract::bootstrap_registry(&state.wallet_info).await {
                    Ok(info) => {
                        state.app.contract_info.set(Some(info));
                    },
                    Err(err) => {
                        state.error.set(Some(err.to_string()));
                    }

                }
            }))
            .class(&*styles::PAGE)
            .children(&mut [
                html!("h1", {
                    .text_signal(state.error.signal_cloned().map(|error| {
                        match error {
                            Some(err) => err,
                            None => "Please wait...".to_string()
                        }
                    }))
                }),
            ])
        })
    }
}