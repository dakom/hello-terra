//Note database limits:
// https://github.com/CosmWasm/cosmwasm/blob/100df08dc3f8f54ab73eb58f56dc0f02aaceca7f/packages/vm/src/imports.rs#L33


use cw_storage_plus::{Item, Map};
use std::collections::HashMap;
use cosmwasm_std::{Addr, Decimal};
use serde::{Deserialize, Serialize};

pub const OWNER: Item<Owner> = Item::new("owner");

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Owner {
    /// Total deposited over entire history 
    pub addr: Addr,
}
/// Accounts are per-coin, like a user having different banking accounts
/// It is not per-user (that's at the contract instantiation level)
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