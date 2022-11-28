use cosmwasm_schema::cw_serde;

/// A definition for a validation service which is stored as a [queriable](crate::contract::query) item in the
/// contract's [storage](crate::storage::request) as the result of [executing](crate::contract::execute)
/// a [request](crate::types::request::validation_definition::ValidationDefinitionCreationRequest).
/// A [ValidationRequest](crate::types::request::validation_request::ValidationRequest) must pertain
/// to exactly one [ValidationDefinition].
#[cw_serde]
pub struct ValidationDefinition {
    /// The type of validation. Used as the [storage](crate::storage::validation_definition) key.
    pub validation_type: String,
    /// An optional display name for the validation definition.
    pub display_name: Option<String>,
    /// Whether new [ValidationRequest](crate::types::request::validation_request::ValidationRequest)s
    /// which use this definition can be created or not. Managed by the contract admin.
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
    pub fn storage_key(&self) -> String {
        self.validation_type.to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::validation_definition::ValidationDefinition;

    use proptest::option::of as option_of;
    use proptest::{prelude::any, prop_assert_eq, proptest};

    proptest! {
        #[test]
        fn set_and_get_validation_definition(
            validation_type in ".+",
            display_name in option_of(".+"),
            enabled in any::<bool>(),
        ) {
            let definition = ValidationDefinition { validation_type: validation_type.clone(), display_name: display_name.clone(), enabled };
            // TODO: Ensure the errors collected by prop_assert are returned somewhere for reporting
            prop_assert_eq!(validation_type.clone(), definition.get_validation_type());
            prop_assert_eq!(display_name, definition.maybe_get_display_name().map(|v| v.to_string()));
            prop_assert_eq!(enabled, definition.enabled);
            prop_assert_eq!(validation_type.to_lowercase(), definition.storage_key());
        }
    }
}
