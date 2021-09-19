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
    /// Derived from both the contract balances and wallet balances
    /// at the time of requesting
    AvailableCoins,

    /// Account info for a specific coin denomination
    AccountSummary(CoinDenom) 
}

/// Return message from [AvilableCoins](QueryMsg::AvailableCoins)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AvailableCoins {
    /// All the available coins for depositing/withdrawing
    pub list: Vec<CoinDenom>,
}

/// The account summary is per-user
/// To get a global total_deposits, query the [Hub Contrat](crate::contracts::hub)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct AccountSummary {
    /// Total balance throughout history of the account 
    pub total_deposits: Decimal, 
    /// Current balance in this account
    pub account_balance: Decimal,
    /// Balance of the user's wallet for this denomination
    pub wallet_balance: Decimal,
}