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
