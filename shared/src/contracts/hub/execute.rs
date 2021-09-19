use cosmwasm_std::{Addr, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Coin;

/// Execute messages for the Hub
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    /// Instantiates a new [Account Contract](crate::contracts::account)
    NewAccount,
    /// Increases the total deposit count for the provided coins 
    AddDeposits(Vec<Coin>),
}


/// Return message from [NewAccount](ExecuteMsg::NewAccount)
/// Contains the address for the instantiated contract 
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NewAccount {
    pub addr: Addr 
}

/// Return message from [AddDeposits](ExecuteMsg::NewAccount)
/// Contains the totals for each coin 
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NewDepositsInfo {
    pub totals: Vec<Coin>
}