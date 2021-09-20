use std::rc::Rc;
use std::cell::RefCell;
use crate::{app::App, utils::wallet_bridge::PostError};
use crate::utils::prelude::*;
use cosmwasm_std::Decimal;
use dominator_helpers::futures::AsyncLoader;
use futures_signals::{map_ref, signal::{Mutable, Signal, SignalExt}};
use super::super::Account;
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

pub struct Manager {
    pub account: Rc<Account>,
    pub coin: CoinDenom,
    pub funds: Mutable<Option<Result<Rc<Funds>, String>>>, 
    pub loader: AsyncLoader,
}

impl Manager {
    pub fn new(account: Rc<Account>, coin: CoinDenom) -> Rc<Self> {
        Rc::new(Self {
            account, 
            coin,
            funds: Mutable::new(None),
            loader: AsyncLoader::new(),
        })
    }

    //if loading any action, just don't let the user mess with funds at all
    //(same if they aren't actually available yet)
    pub fn funds_signal(&self) -> impl Signal<Item = Option<Result<Rc<Funds>, String>>> {
        map_ref! {
            let funds = self.funds.signal_cloned(),
            let loader_loading = self.loader.is_loading()
                => {
                    if *loader_loading {
                        None
                    } else {
                        funds.clone()
                    }
                }
        }
    }
}

pub struct Funds {
    pub account_balance: Decimal,
    pub wallet_balance: Decimal,
    pub account_deposit_total: Decimal,
    pub global_deposit_total: Decimal,
    pub input_error: Mutable<Option<String>>,
    pub input_deposit_value: RefCell<Option<String>>,
    pub input_withdrawal_value: RefCell<Option<String>>,
}

impl Funds {
    pub fn new(global_deposit: hub_query::TotalDeposited, account_summary: account_query::AccountSummary) -> Rc<Self> {
        Rc::new(Self {
            account_balance: account_summary.account_balance,
            wallet_balance: account_summary.wallet_balance,
            account_deposit_total: account_summary.total_deposits,
            global_deposit_total: global_deposit.total,
            input_error: Mutable::new(None),
            input_deposit_value: RefCell::new(None),
            input_withdrawal_value: RefCell::new(None),
        })
    }
}