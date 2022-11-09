use super::{entity::EntityDetail, validation_cost::ValidationCost};

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct ValidatorConfiguration {
    pub validation_costs: Vec<ValidationCost>,
    pub validation_type: String,
    pub validator: EntityDetail,
}
impl ValidatorConfiguration {
    pub fn get_validation_type(&self) -> &str {
        &self.validation_type
    }
    pub fn get_validation_costs(&self) -> &[ValidationCost] {
        &self.validation_costs
    }
}
