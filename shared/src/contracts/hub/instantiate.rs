use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Entry point of the whole system
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg { 
    pub account_code_id: u64 
}

impl InstantiateMsg {
    pub fn new(account_code_id: u64) -> Self {
        Self { account_code_id }
    }
}
