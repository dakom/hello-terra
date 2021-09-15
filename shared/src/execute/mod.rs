use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub mod summary;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    FullSummary(summary::FullSummaryRequest)
}