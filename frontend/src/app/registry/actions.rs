use std::{rc::Rc, cell::RefCell};
use super::state::*;
use web_sys::{File, FileReader};
use crate::utils::{prelude::*, terra::TERRA};
use gloo_events::EventListener;
use dominator::clone;
use futures::channel::oneshot;
use wasm_bindgen::JsCast;
use js_sys::{ArrayBuffer, Uint8Array};
impl Registry {
    pub fn upload_contract_file(state: Rc<Self>, file: File) {
        state.loader.load(clone!(state => async move {
            let reader = FileReader::new().unwrap_ext();

            let (sender, receiver) = oneshot::channel::<Option<String>>();
            let sender = Rc::new(RefCell::new(Some(sender)));

            let listener = Rc::new(RefCell::new(None));
            
            *listener.borrow_mut() = Some(
                EventListener::new(&reader, "load", clone!(listener, sender, state => move |event| {
                    let reader:FileReader = event.target().unwrap_ext().unchecked_into();
                    let mut result:Option<String> = None;

                    if let Ok(value) = reader.result() {
                        let buffer:ArrayBuffer = value.unchecked_into();
                        let bytes = Uint8Array::new(&buffer).to_vec();
                        if &bytes[0..4] == [0x00, 0x61, 0x73, 0x6D] {
                            result = Some(base64::encode(&bytes));
                        } else {
                            web_sys::window().unwrap_ext().alert_with_message("invalid WASM file!");
                        }
                    }

                    sender.borrow_mut().take().unwrap_ext().send(result);
                    listener.borrow_mut().take();
                }))
            );

            reader.read_as_array_buffer(&file);

            match receiver.await {
                Ok(result) => {
                    if let Some(byte_string) = result {
                        let (sender, receiver) = oneshot::channel::<Option<String>>();
                        *state.contract_id_sender.borrow_mut() = Some(sender);
                        WalletMsg::Request(WalletRequest::ContractUpload(byte_string)).post();

                        if let Some(id) = receiver.await.ok().and_then(|result| result) {
                            log::info!("GOT CONTRACT ID: {}", id);
                        } else {
                            web_sys::window().unwrap_ext().alert_with_message("unable to upload contract!");
                        }
                    }
                },
                _ => { }
            }
        }));
    }

    pub fn handle_wallet_message(state: Rc<Self>, msg: WalletMsg) {
        match msg {
            WalletMsg::Response(resp) => {
                match resp {
                    WalletResponse::ContractUpload(id) => {
                        if let Some(sender) = state.contract_id_sender.borrow_mut().take() {
                            sender.send(id);
                        }
                    },
                    _ => {}
                }
            },
            _ => {}

        }
    }
}
