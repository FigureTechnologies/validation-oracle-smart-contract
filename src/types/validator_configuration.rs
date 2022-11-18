use super::validation_cost::ValidationCost;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct ValidatorConfiguration {
    pub validation_costs: Vec<ValidationCost>,
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
