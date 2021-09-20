use std::{ops::Add, rc::Rc};
use std::collections::HashSet;
use cosmwasm_std::{Addr, Coin, Decimal};
use dominator::{html, Dom, clone, with_node};
use shared::contracts::account::execute::ExecuteMsg;
use super::{state::*, styles};
use crate::utils::wallet_bridge::PostError;
use crate::{components::{button::*, image::*, overlay::Overlay}, utils::wallet_bridge::ContractQueryMsg};
use futures_signals::signal::SignalExt;
use crate::utils::prelude::*;
use shared::{
    coin::CoinDenom, 
    contracts::{
        hub::{
            query as hub_query,
            execute as hub_execute
        },
        account::{
            query as account_query,
            execute as account_execute
        }
    }
};

impl Manager {
    pub async fn reload(&self) {
        match self.try_reload().await {
            Ok(funds) => {
                self.funds.set(Some(Ok(funds)));
            },
            Err(err) => {
                self.funds.set(Some(Err(format!("error: {:?}", err.to_string()))));
            }
        }
    }

    async fn try_reload(&self) -> Result<Rc<Funds>, PostError> {

        let global_deposit = ContractQueryMsg {
            addr: self.hub_addr(),
            msg: hub_query::QueryMsg::TotalDeposited(self.coin.clone())
        }.query::<hub_query::TotalDeposited>().await?;

        let account_summary = ContractQueryMsg {
            addr: self.account_addr(),
            msg: account_query::QueryMsg::AccountSummary(self.coin.clone())
        }.query::<account_query::AccountSummary>().await?;

        Ok(Funds::new(global_deposit, account_summary))
    }

    fn hub_addr(&self) -> String {
        self.account.contract_info.hub_addr.clone()
    }

    fn account_addr(&self) -> String {
        self.account.contract_info.account_addr.clone()
    }

    pub fn do_deposit(state: Rc<Self>, funds: Rc<Funds>) {
        let amount = sanitize_amount(
            &funds.input_deposit_value.borrow(),
            &funds
        );

        if let Some(amount) = amount {
            state.loader.load(clone!(state, funds => async move {
                let resp = ContractExecuteMsg {
                    addr: state.account_addr(),
                    msg: account_execute::ExecuteMsg::Deposit,
                    coins: Some(vec![Coin::new(amount, state.coin.clone())])
                }.execute_noresp().await;

                if let Err(err) = resp {
                    log::info!("{:?}", err);
                    funds.input_error.set(Some("unable to process deposit!".to_string()));
                } else {
                    state.reload().await;
                }
            }));
        }
    }
    pub fn do_withdrawal(state: Rc<Self>, funds: Rc<Funds>) {
        let amount = sanitize_amount(
            &funds.input_withdrawal_value.borrow(),
            &funds
        );

        if let Some(amount) = amount {
            state.loader.load(clone!(state, funds => async move {
                let resp = ContractExecuteMsg {
                    addr: state.account_addr(),
                    msg: account_execute::ExecuteMsg::Withdraw(Coin::new(amount, state.coin.clone())),
                    coins: None, 
                }.execute_noresp().await;

                if let Err(err) = resp {
                    funds.input_error.set(Some("unable to process withdrawal!".to_string()));
                } else {
                    state.reload().await;
                }
            }));
        }
    }

}

fn sanitize_amount<T: AsRef<str>>(amount:&Option<T>, funds:&Funds) -> Option<u128> {
    let amount = amount.as_ref().and_then(|amount| {
        amount.as_ref().parse::<u128>().ok()
    });

    if amount.is_none() {
        funds.input_error.set(Some("set a valid amount".to_string()));
    }

    amount
}