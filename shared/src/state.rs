use std::collections::HashMap;

use cosmwasm_std::{Addr, Decimal};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Totals per-coin
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Account {
    pub total_deposits: Decimal,
    pub balance: Decimal,
}