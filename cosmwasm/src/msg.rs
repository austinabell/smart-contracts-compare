use crate::state::ContentRecord;
use cosmwasm_std::Coin;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Empty data to initialize contract.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

/// Message for performing a state transition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    /// User sends token to purchase a route.
    Purchase { route: String, content: String },
    /// Allows contract owner to withdraw funds.
    Withdraw {},
}

/// Queries defined for state.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Queries route and returns content.
    GetRoute { route: String },
}

/// Response type for [QueryMsg::GetRoute].
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContentResponse {
    pub content: String,
    pub price: Coin,
}

impl From<ContentRecord> for ContentResponse {
    fn from(record: ContentRecord) -> Self {
        Self {
            content: record.content,
            price: record.price,
        }
    }
}
