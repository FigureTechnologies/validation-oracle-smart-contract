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

#[cw_serde]
pub struct ValidationDefinitionUpdateRequest {
    pub current_validation_type: String,
    pub new_validation_type: Option<String>,
    pub new_display_name: Option<String>,
    pub enabled: Option<bool>,
    //pub bind_name: Option<bool>, // TODO: How to let them un/re/bind the name? Or maybe let it be more explicit via new_bind_name field?
}
impl ValidationDefinitionUpdateRequest {
    pub fn get_current_validation_type(&self) -> &str {
        &self.current_validation_type
    }
    pub fn maybe_get_new_validation_type(&self) -> Option<&str> {
        self.new_validation_type.as_deref()
    }
    pub fn get_new_validation_type(&self) -> &str {
        self.new_validation_type.as_ref().unwrap()
    }
    pub fn maybe_get_display_name(&self) -> Option<&str> {
        self.new_display_name.as_deref()
    }
    pub fn get_display_name(&self) -> &str {
        self.new_display_name.as_ref().unwrap()
    }
    // TODO: How do we make storage keys more maintainable (look at hardcoded error msgs for why)?
    pub fn old_storage_key(&self) -> String {
        self.current_validation_type.to_lowercase()
    }
    // TODO: How to return an Option<&str> here?
    pub fn maybe_get_new_storage_key(&self) -> Option<String> {
        self.new_validation_type.clone().map(|s| s.to_lowercase())
    }
    pub fn get_new_storage_key(&self) -> String {
        self.new_validation_type.clone().unwrap().to_lowercase()
    }
}
impl From<ValidationDefinitionUpdateRequest> for ValidationDefinition {
    fn from(request: ValidationDefinitionUpdateRequest) -> Self {
        ValidationDefinition {
            validation_type: request
                .new_validation_type
                .unwrap_or(request.current_validation_type),
            display_name: request.new_display_name,
            enabled: request.enabled.unwrap_or(true),
        }
    }
}
