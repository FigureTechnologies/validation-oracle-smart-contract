use super::validation_cost::ValidationCost;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct ValidatorConfiguration {
    pub validation_costs: Vec<ValidationCost>, // TODO: Sort Vecs before storing them?
    pub validation_type: String,
    pub validator: Addr,
}
impl ValidatorConfiguration {
    pub fn get_validation_type(&self) -> &str {
        &self.validation_type
    }
    pub fn get_validation_costs(&self) -> &[ValidationCost] {
        &self.validation_costs
    }
    pub fn storage_key(&self) -> String {
        format!("{}-{}", &self.validator, &self.get_validation_type())
    }
}

#[cfg(test)]
mod tests {
    use crate::test::arbitrary::{arb_addr, arb_validation_cost, arb_validation_type};
    use crate::types::validator_configuration::ValidatorConfiguration;

    use proptest::collection::vec;
    use proptest::sample::size_range;
    use proptest::strategy::{Just, Strategy};
    use proptest::{prop_assert_eq, proptest};

    proptest! {
        #[test]
        fn set_and_get_validation_configuration(
            (validator, validation_costs) in arb_addr().prop_flat_map(|addr| (Just(addr.clone()), vec(arb_validation_cost(Some(addr)), size_range(1..100)))),
            validation_type in arb_validation_type(),
        ) {
            let configuration = ValidatorConfiguration { validation_type: validation_type.clone(), validation_costs: validation_costs.clone(), validator: validator.clone() };
            prop_assert_eq!(validator.clone(), configuration.validator.clone());
            prop_assert_eq!(validation_type.clone(), configuration.get_validation_type());
            prop_assert_eq!(validation_costs, configuration.get_validation_costs());
        }
    }
}
