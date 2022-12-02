use super::entity::EntityDetail;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct ValidationCost {
    //pub fee: Coin,
    pub amount: Uint128,
    pub denom: String,
    pub destination: EntityDetail, // TODO: Change to key by address (i.e. support exactly zero or one entity detail(s) per address)
}
impl ValidationCost {
    pub fn get_amount(&self) -> u128 {
        self.amount.u128()
    }
    pub fn get_denom(&self) -> &str {
        &self.denom
    }
}

#[cfg(test)]
mod tests {
    use crate::test::arbitrary::{arb_coin, arb_entity};
    use crate::types::validation_cost::ValidationCost;

    use proptest::{prop_assert_eq, proptest};

    proptest! {
        #[test]
        fn set_and_get_validation_cost(
            fee in arb_coin(),
            destination in arb_entity(None),
        ) {
            let cost = ValidationCost { amount: fee.amount.clone(), denom: fee.denom.clone(), destination: destination.clone() };
            prop_assert_eq!(fee.amount.u128(), cost.get_amount());
            prop_assert_eq!(destination, cost.destination);
        }
    }
}
