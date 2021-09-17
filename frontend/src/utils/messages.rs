
/*
pub trait ExecutePostMsg {
    fn post(self, addr: &str, coins: Option<Coins>) where Self: Sized {
        self.try_post(addr, coins).unwrap_ext();
    }

    fn try_post(self, addr: &str, coins: Option<Coins>) -> Result<(), JsValue>;
}

impl ExecutePostMsg for shared::execute::ExecuteMsg {
    fn try_post(self, addr: &str, coins: Option<Coins>) -> Result<(), JsValue> {

        ContractExecuteMsg {
            addr: addr.to_string(),
            coins,
            msg: self 
        }.try_post()
    }
}
*/