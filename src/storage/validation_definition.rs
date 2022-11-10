use crate::{
    types::{core::error::ContractError, validation_definition::ValidationDefinition},
    util::aliases::ContractResult,
};

use cosmwasm_std::Storage;
use cw_storage_plus::Map;
use result_extensions::ResultExtensions;

const NAMESPACE_VALIDATION_DEFINITIONS: &str = // TODO: Investigate further
    concat!("validation_definitions_", env!("CARGO_PKG_VERSION")); // Alternative: use crate const_concat

const VALIDATION_DEFINITIONS: Map<String, ValidationDefinition> =
    Map::new(NAMESPACE_VALIDATION_DEFINITIONS);

/// Inserts a validation definition into the contract's storage, returning
/// a [Result] reflecting whether the insertion succeeded or not.
///
/// # Parameters
///
/// * `storage` A mutable reference to the contract's internal storage.
/// * `request` The validation definition to insert.
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
        state
            .save(storage, key, definition)
            .map_err(|e| ContractError::StorageError {
                message: format!("{:?}", e),
            })
    }
}

/// Returns a validation definition from the contract's storage by its key,
/// returning a [Result] reflecting whether the retrieval succeeded or not.
///
/// # Parameters
///
/// * `storage` A reference to the contract's internal storage.
/// * `key` A storage key for a validation definition.
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

/// Returns a validation definition from the contract's storage by its key, returning
/// an [Option] reflecting whether the validation definition was found or not.
///
/// # Parameters
///
/// * `storage` A reference to the contract's internal storage.
/// * `key` A storage key for a validation definition.
pub fn may_get_validation_definition<S: Into<String>>(
    storage: &dyn Storage,
    key: S,
) -> Option<ValidationDefinition> {
    VALIDATION_DEFINITIONS
        .may_load(storage, key.into())
        .unwrap_or(None)
}
