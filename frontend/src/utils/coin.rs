use serde::{Serialize, Deserialize, de::DeserializeOwned};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::collections::HashMap;
use std::fmt::Debug;

use super::prelude::MyUnwrapExt;


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "kind", content = "data")]
pub enum Coins {
    Multi(Vec<Coin>),
    Single(Coin),
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Coin {
    pub denom: CoinDenom,
    pub amount: f64
}


#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CoinDenom {
    Luna,
    Ust
}