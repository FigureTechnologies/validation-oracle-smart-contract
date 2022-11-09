use super::entity::EntityDetail;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct ValidationCost {
    //pub fee: Coin,
    pub amount: Uint128,
    pub denom: String,
    pub destination: EntityDetail,
}
impl ValidationCost {
    pub fn get_denom(&self) -> &str {
        &self.denom
    }
}
