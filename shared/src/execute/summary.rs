use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Initialization request to get the initial
/// summary of all the funds
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FullSummaryRequest {
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct FullSummaryResponse {
    /// amount of money in the wallet
    /// not specific to this app, rather it's across the whole chain
    pub wallet_balance: f64,
    /// amount of money in this account/app
    pub deposit_balance: f64,
    /// history of total spent in this account/app
    pub total_deposits: f64,

}