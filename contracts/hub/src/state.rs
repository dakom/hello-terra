//Note database limits:
// https://github.com/CosmWasm/cosmwasm/blob/100df08dc3f8f54ab73eb58f56dc0f02aaceca7f/packages/vm/src/imports.rs#L33


use cw_storage_plus::{Item, Map};
use cosmwasm_std::{Addr, Decimal};
use serde::{Deserialize, Serialize};

/// Mapping of user to contract addr 
pub const USERS: Map<&[u8], Addr> = Map::new("users");
/// Total deposits per-coin 
pub const TOTALS: Map<&[u8], Decimal> = Map::new("totals");