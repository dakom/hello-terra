//Note database limits:
// https://github.com/CosmWasm/cosmwasm/blob/100df08dc3f8f54ab73eb58f56dc0f02aaceca7f/packages/vm/src/imports.rs#L33


use cw_storage_plus::{Item, Map};
use cosmwasm_std::{Addr, Decimal};
use serde::{Deserialize, Serialize};

/// Account contract code ID for instantiation
pub const ACCOUNT_CODE_ID: Item<u64> = Item::new("account_code_id");

/// Mapping of user to contract addr 
pub const USERS: Map<&[u8], Addr> = Map::new("users");

/// Reverse-mapping of contract addr to user 
pub const CONTRACTS: Map<&[u8], Addr> = Map::new("contracts");

/// Total deposits per-coin 
pub const TOTALS: Map<&[u8], Decimal> = Map::new("totals");