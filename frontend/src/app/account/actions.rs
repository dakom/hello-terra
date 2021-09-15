use std::{cell::RefCell, fmt::Error, rc::Rc};
use super::state::*;
use crate::utils::{prelude::*, storage::*};
use futures_signals::signal::Mutable;
use dominator::clone;
use futures::channel::oneshot;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;

impl Account {

    pub fn handle_loading_message(state: Rc<Self>, msg: WalletMsg) {
        match msg {
            WalletMsg::Response(resp) => {
                match resp {
                    WalletResponse::ContractExecute(resp) => {
                        if let Some(resp) = resp {
                            match resp {
                                ContractExecuteResp::FullSummaryResponse(resp) => {
                                    state.funds.set(Some(Rc::new(Funds {
                                        wallet_balance: Mutable::new(resp.wallet_balance),
                                        total_deposits: Mutable::new(resp.total_deposits),
                                        deposit_balance: Mutable::new(resp.deposit_balance)
                                    })));
                                }
                            }
                        }

                        //replace funds...
                    },
                    _ => {}
                }
            },
            _ => {}

        }
    }
}
