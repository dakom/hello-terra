use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::coin::CoinDenom;

/// Send Query messages to read state
/// This is quicker and consumes less gas than an [ExecuteMsg](super::execute::ExecuteMsg)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(tag = "kind", content = "data")]
pub enum QueryMsg {
    /// List all the available coins for depositing/withdrawing
    /// Derived from only the history of global deposits
    /// Should be mixed with per-user wallet info for maximum results 
    AvailableCoins,

    /// Total deposited by all users for a specific coin
    TotalDeposited(CoinDenom),
}

/// Return message from [AvailableCoins](QueryMsg::AvailableCoins)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AvailableCoins {
    pub list: Vec<CoinDenom>,
}

/// Return message from [TotalDeposited](QueryMsg::TotalDeposited)
/// Contains the total deposited for the requested coin for all users
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TotalDeposited {
    pub total: Decimal 
}