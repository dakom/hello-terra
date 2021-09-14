use std::rc::Rc;
use crate::app::App;
use crate::utils::prelude::*;

pub struct Account {
    pub wallet_info: WalletInfo,
    pub contract_info: ContractInfo,
    pub app: Rc<App> 
}

impl Account {
    pub fn new(wallet_info:WalletInfo, contract_info: ContractInfo, app: Rc<App>) -> Rc<Self> {
        Rc::new(Self {
            app,
            wallet_info,
            contract_info,
        })
    }
}