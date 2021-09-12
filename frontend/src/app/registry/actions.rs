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
                        Self::upload_byte_string(state, byte_string).await;
                    }
                },
                _ => { }
            }
        }));
    }

    async fn upload_byte_string(state: Rc<Self>, byte_string:String) {
        match TERRA.upload_contract(&byte_string).await {
            Ok(contract_id) => {
                log::info!("got contract id: {}", contract_id);
            },
            Err(error) => {
                log::error!("{}", error);
                web_sys::window().unwrap_ext().alert_with_message("unable to upload contract (see logs)!");
            }
        }
    }
}
