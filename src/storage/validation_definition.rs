use crate::{
    types::{core::error::ContractError, validation_definition::ValidationDefinition},
    util::aliases::ContractResult,
};

use cosmwasm_std::Storage;
use cw_storage_plus::Map;
use result_extensions::ResultExtensions;

const NAMESPACE_VALIDATION_DEFINITIONS: &str =
    concat!("validation_definitions_", env!("CARGO_PKG_VERSION")); // Alternative: use crate const_concat

const VALIDATION_DEFINITIONS: Map<String, ValidationDefinition> =
    Map::new(NAMESPACE_VALIDATION_DEFINITIONS);

pub fn insert_validation_definition(
    storage: &mut dyn Storage,
    definition: &ValidationDefinition,
) -> ContractResult<()> {
    let state = VALIDATION_DEFINITIONS;
    let key = definition.storage_key();
    if let Ok(existing_definition) = state.load(storage, key.clone()) {
        ContractError::RecordAlreadyExists {
            explanation: format!(
                "unique constraints violated! record with validation type '{}' already exists",
                existing_definition.validation_type
            ),
        }
        .to_err()
    } else {
        // At this point, we know there is no old data available, so we can safely call the replace function and
        // specify None for the old_data param.
        state
            .save(storage, key, definition)
            .map_err(|e| ContractError::StorageError {
                message: format!("{:?}", e),
            })
    }
}

pub fn get_validation_definition<S: Into<String>>(
    storage: &dyn Storage,
    key: S,
) -> ContractResult<ValidationDefinition> {
    VALIDATION_DEFINITIONS
        .load(storage, key.into())
        .map_err(|e| ContractError::StorageError {
            message: format!("{:?}", e),
        })
}

pub fn may_get_validation_definition<S: Into<String>>(
    storage: &dyn Storage,
    key: S,
) -> Option<ValidationDefinition> {
    VALIDATION_DEFINITIONS
        .may_load(storage, key.into())
        .unwrap_or(None)
}
