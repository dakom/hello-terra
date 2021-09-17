use std::{cell::RefCell, fmt::Error, rc::Rc};
use super::state::*;
use web_sys::{File, FileReader};
use crate::utils::{prelude::*, storage::*, contract};
use gloo_events::EventListener;
use dominator::clone;
use futures::channel::oneshot;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use js_sys::{ArrayBuffer, Uint8Array};
use awsm_web::loaders::fetch::{Response, fetch_url};

impl Registry {
    pub fn upload_file_remote(state: Rc<Self>) {
        state.loader.load(clone!(state => async move {
            let file_hash = state.contract_hash.get_cloned().unwrap_ext();

            if let Some(contract_id) = contract::upload_contract_remote(&state.wallet_info, &file_hash).await {
                state.contract_id.set(Some(contract_id));
            }
        }));
    }

    pub fn upload_file_manually(state: Rc<Self>, file:File) {
        state.loader.load(clone!(state, file => async move {
            let file_hash = state.contract_hash.get_cloned().unwrap_ext();

            if let Some(contract_id) = contract::upload_contract_file(&state.wallet_info, &file_hash, file).await {
                state.contract_id.set(Some(contract_id));
            }
        }));
    }

    pub fn instantiate_contract(state: Rc<Self>, contract_id: u64) {
        state.loader.load(clone!(state, contract_id => async move {
            if let Some(contract_info) = contract::instantiate_contract(&state.wallet_info, contract_id.clone(), None::<()>).await {
                state.app.contract_info.set(Some(contract_info));
            }
        }));
    }
}
