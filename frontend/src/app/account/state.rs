use std::rc::Rc;
use crate::app::App;
use crate::utils::prelude::*;
use futures_signals::signal::Mutable;

pub struct Account {
    pub wallet_info: WalletInfo,
    pub contract_info: ContractInfo,
    pub app: Rc<App>,
    pub funds: Mutable<Option<Rc<Funds>>>,
}

pub struct Funds {
    pub wallet_balance: Mutable<f64>,
    pub deposit_balance: Mutable<f64>,
    pub total_deposits: Mutable<f64>,
}

impl Account {
    pub fn new(wallet_info:WalletInfo, contract_info: ContractInfo, app: Rc<App>) -> Rc<Self> {
        Rc::new(Self {
            app,
            wallet_info,
            contract_info,
            funds: Mutable::new(None)
        })
    }
}