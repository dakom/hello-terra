use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::coin::CoinDenom;

/// Send Query messages to read state
/// This is quicker and consumes less gas than an [ExecuteMsg](super::execute::ExecuteMsg)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(tag = "kind", content = "data")]
pub enum QueryMsg {
    /// Total deposited by all users for a specific coin
    TotalDeposited(CoinDenom),
}

/// Return message from [TotalDeposited](QueryMsg::TotalDeposited)
/// Contains the total deposited for the requested coin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TotalDeposited {
    pub total: Decimal 
}