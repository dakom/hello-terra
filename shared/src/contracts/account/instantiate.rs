use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The account contract shouldn't be instantiated directly
/// Rather, it gets instantiated by the [Hub Contract](crate::contracts::hub) 
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// The address of the user who initiated things
    /// It's auto-derived via the Hub's [MessageInfo.sender](https://docs.rs/cosmwasm-std/0.16.2/cosmwasm_std/struct.MessageInfo.html)
    pub owner_addr: Addr
}

impl InstantiateMsg {
    pub fn new(owner_addr: Addr) -> Self {
        Self { owner_addr }
    }
}
