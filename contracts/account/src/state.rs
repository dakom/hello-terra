//Note database limits:
// https://github.com/CosmWasm/cosmwasm/blob/100df08dc3f8f54ab73eb58f56dc0f02aaceca7f/packages/vm/src/imports.rs#L33


use cw_storage_plus::{Item, Map};
use cosmwasm_std::{Addr, Decimal};

pub const HUB: Item<Addr> = Item::new("hub");
pub const OWNER: Item<Addr> = Item::new("owner");
/// this is per-coin, like different bank accounts
/// It is not per-user (that's at the contract instantiation level)
/// To get aggregates across the entire system, query the Hub
pub const TOTAL_DEPOSITS: Map<&[u8], Decimal> = Map::new("accounts");