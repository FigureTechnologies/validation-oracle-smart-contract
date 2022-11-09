use super::validator_configuration::ValidatorConfiguration;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct ValidationDefinition {
    pub validation_type: String,
    pub display_name: Option<String>,
    pub validators: Vec<ValidatorConfiguration>,
    pub enabled: bool,
}
impl ValidationDefinition {
    pub fn get_validation_type(&self) -> &str {
        &self.validation_type
    }
    pub fn maybe_get_display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }
    pub fn get_display_name(&self) -> &str {
        self.display_name.as_ref().unwrap()
    }
    pub fn get_validators(&self) -> &[ValidatorConfiguration] {
        &self.validators
    }
    pub fn storage_key(&self) -> String {
        self.validation_type.to_lowercase()
    }
}
