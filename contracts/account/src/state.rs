//Note database limits:
// https://github.com/CosmWasm/cosmwasm/blob/100df08dc3f8f54ab73eb58f56dc0f02aaceca7f/packages/vm/src/imports.rs#L33


use cw_storage_plus::{Item, Map};
use cosmwasm_std::{Addr, Decimal};
use serde::{Deserialize, Serialize};

pub const HUB: Item<Addr> = Item::new("hub");
pub const OWNER: Item<Addr> = Item::new("owner");
/// Accounts are per-coin, like a user having different banking accounts
/// It is not per-user (that's at the contract instantiation level)
/// To get aggregates across the entire system, query the Hub
pub const ACCOUNTS: Map<&[u8], Account> = Map::new("accounts");

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Account {
    /// Total deposited over entire history 
    pub total_deposits: Decimal,

    /// Current balance 
    pub balance: Decimal,
}

impl Account {
    pub fn deposit(&mut self, amount: Decimal) {
        //Decimal doesn't have +=
        self.total_deposits = self.total_deposits + amount;
        self.balance = self.balance + amount;
    }

    pub fn withdraw(&mut self, amount: Decimal) -> bool {
        if self.balance >= amount {
            //Decimal doesn't have -=
            self.balance = self.balance - amount;
            true
        } else {
            false
        }
    }
}