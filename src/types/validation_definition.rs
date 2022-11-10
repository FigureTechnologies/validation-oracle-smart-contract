use super::validator_configuration::ValidatorConfiguration;

use cosmwasm_schema::cw_serde;

/// A definition for a validation service which is stored as a [queriable](crate::contract::query) item in the
/// contract's [storage](crate::storage::request_storage) as the result of [executing](crate::contract::execute)
/// a [request](crate::types::request::validation_definition::ValidationDefinitionCreationRequest).
/// A [ValidationRequest] must pertain to exactly one [ValidationDefinition].
#[cw_serde]
pub struct ValidationDefinition {
    /// The type of validation. Used as the [storage](crate::storage::validation_definition) key.
    pub validation_type: String,
    /// An optional display name for the validation definition.
    pub display_name: Option<String>,
    /// A list of validator configurations.
    pub validators: Vec<ValidatorConfiguration>,
    /// Whether new [ValidationRequest]s which use this definition can be created or not. Managed by
    /// the contract admin.
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
