use cosmwasm_std::{Addr, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Coin;

/// Deposit or withdraw money
/// There's no return message because a successful transaction
/// Means that it's accurate for the UI to update values locally
/// Alternatively, a separate [AccountSummary](crate::QueryMsg::AccountSummary) query could be issued
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum ExecuteMsg {
    Deposit,
    Withdraw(Coin)
}
