use std::rc::Rc;
use crate::app::App;
use crate::utils::prelude::*;
use futures_signals::signal::Mutable;
use shared::coin::CoinDenom;

pub struct Account {
    pub wallet_info: WalletInfo,
    pub contract_info: ContractInfo,
    pub app: Rc<App>,
    pub available_coins: Mutable<Option<Vec<CoinDenom>>>,
    pub selected_coin: Mutable<Option<CoinDenom>>,
}

impl Account {
    pub fn new(wallet_info:WalletInfo, contract_info: ContractInfo, app: Rc<App>) -> Rc<Self> {
        Rc::new(Self {
            app,
            wallet_info,
            contract_info,
            available_coins: Mutable::new(None),
            selected_coin: Mutable::new(None),
        })
    }
}