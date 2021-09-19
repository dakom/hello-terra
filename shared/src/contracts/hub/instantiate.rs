use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Entry point of the whole system
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg { }
