use crate::types::validation_definition::ValidationDefinition;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct ValidationDefinitionCreationRequest {
    pub validation_type: String,
    pub display_name: Option<String>,
    pub enabled: Option<bool>,
    pub bind_name: Option<bool>,
}
impl ValidationDefinitionCreationRequest {
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
}
impl From<ValidationDefinitionCreationRequest> for ValidationDefinition {
    fn from(request: ValidationDefinitionCreationRequest) -> Self {
        ValidationDefinition {
            validation_type: request.validation_type,
            display_name: request.display_name,
            enabled: request.enabled.unwrap_or(true),
        }
    }
}
