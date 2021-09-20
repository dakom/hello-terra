use cosmwasm_std::{Addr, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Coin;

/// Execute messages for the Hub
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    /// Gets or Instantiates a new [Account Contract](crate::contracts::account)
    /// Will return the address in [AccountInfo]
    GetAccountInfo,
    /// Increases the total deposit count for the provided coins 
    AddDeposits(Vec<Coin>),
}


/// Return message from [NewAccount](ExecuteMsg::NewAccount)
/// Contains the address for the instantiated contract 
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AccountInfo {
    pub contract_addr: Addr 
}