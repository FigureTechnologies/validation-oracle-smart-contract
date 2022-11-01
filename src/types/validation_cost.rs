use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::entity::EntityDetail;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ValidationCost {
    //pub fee: Coin,
    pub amount: Uint128,
    pub denom: String,
    pub destination: EntityDetail,
}
