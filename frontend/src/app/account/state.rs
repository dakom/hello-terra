use std::rc::Rc;
use crate::app::App;

pub struct Account {
    pub wallet_id: String,
    pub contract_id: String,
    pub app: Rc<App> 
}

impl Account {
    pub fn new(wallet_id:String, contract_id: String, app: Rc<App>) -> Rc<Self> {
        Rc::new(Self {
            app,
            wallet_id,
            contract_id,
        })
    }
}