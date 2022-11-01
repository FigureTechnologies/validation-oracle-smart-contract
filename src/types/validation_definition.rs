use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::validator_configuration::ValidatorConfiguration;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ValidationDefinition {
    pub validation_type: String,
    pub display_name: Option<String>,
    pub validators: Vec<ValidatorConfiguration>,
    pub enabled: bool,
}
