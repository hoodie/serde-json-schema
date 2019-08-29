use serde::{Deserialize, Serialize};

/// Number validation Criteria (WIP)
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NumberCriteria {
    exclusive_minimum: Option<serde_json::Value>,
}