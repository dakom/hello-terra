use std::rc::Rc;
use crate::app::App;

pub struct Account {
    pub wallet_addr: String,
    pub contract_addr: String,
    pub app: Rc<App> 
}

impl Account {
    pub fn new(wallet_addr:String, contract_addr: String, app: Rc<App>) -> Rc<Self> {
        Rc::new(Self {
            app,
            wallet_addr,
            contract_addr,
        })
    }
}