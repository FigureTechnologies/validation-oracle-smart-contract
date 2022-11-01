use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{entity::EntityDetail, validation_cost::ValidationCost};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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
