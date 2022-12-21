use crate::{
    types::{core::error::ContractError, validator_configuration::ValidatorConfiguration},
    util::aliases::ContractResult,
};

use cosmwasm_std::Storage;
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use result_extensions::ResultExtensions;

/// The primary key prefix for the storage of [ValidatorConfiguration]s in an [IndexedMap].
const NAMESPACE_VALIDATOR_CONFIGURATIONS_PK: &str = "validator_configuration";
/// The prefix in the [DefinitionIndices] for indexing [ValidatorConfiguration]s by their validation type.
const NAMESPACE_VALIDATOR_CONFIGURATIONS_TYPE_IDX: &str = "validator_configuration__type";

/// Defines a collection of [MultiIndex]s for storing [ValidatorConfiguration]s in
/// a shared primary key namespace.
pub struct DefinitionIndices<'a> {
    pub type_index: MultiIndex<'a, String, ValidatorConfiguration, String>,
}
impl<'a> IndexList<ValidatorConfiguration> for DefinitionIndices<'a> {
    fn get_indexes(
        &'_ self,
    ) -> Box<dyn Iterator<Item = &'_ dyn Index<ValidatorConfiguration>> + '_> {
        let v: Vec<&dyn Index<ValidatorConfiguration>> = vec![&self.type_index];
        Box::new(v.into_iter())
    }
}

/// Returns the contract's storage of validator configurations.
fn validator_configurations<'a>(
) -> IndexedMap<'a, &'a [u8], ValidatorConfiguration, DefinitionIndices<'a>> {
    let indices = DefinitionIndices {
        type_index: MultiIndex::new(
            |_pk, configuration: &ValidatorConfiguration| {
                configuration.get_validation_type().to_string()
            },
            NAMESPACE_VALIDATOR_CONFIGURATIONS_PK,
            NAMESPACE_VALIDATOR_CONFIGURATIONS_TYPE_IDX,
        ),
    };
    IndexedMap::new(NAMESPACE_VALIDATOR_CONFIGURATIONS_PK, indices)
}

/// Inserts a validator configuration into the contract's storage, returning
/// a [Result] reflecting whether the insertion succeeded or not.
///
/// # Parameters
///
/// * `storage` A mutable reference to the contract's internal storage.
/// * `configuration` The validator configuration to insert.
pub fn insert_validator_configuration(
    storage: &mut dyn Storage,
    configuration: &ValidatorConfiguration,
) -> ContractResult<()> {
    let state = validator_configurations();
    let key = configuration.storage_key();
    if let Ok(existing_configuration) = state.load(storage, key.as_bytes()) {
        ContractError::RecordAlreadyExists {
            explanation: format!(
                "a configuration with validation type [{}] for validator [{}] already exists",
                existing_configuration.get_validation_type(),
                existing_configuration.validator
            ),
        }
        .to_err()
    } else {
        store_validator_configuration(storage, configuration, None)
    }
}

/// Updates an existing validator configuration within the contract's storage,
/// returning a [Result] reflecting whether the insertion succeeded or not.
///
/// # Parameters
///
/// * `storage` A mutable reference to the contract's internal storage.
/// * `configuration` The new validator configuration to replace the one in storage with the same key.
pub fn update_validator_configuration(
    storage: &mut dyn Storage,
    configuration: &ValidatorConfiguration,
) -> ContractResult<()> {
    let state = validator_configurations();
    if let Ok(old_configuration) = state.load(storage, configuration.storage_key().as_bytes()) {
        store_validator_configuration(storage, configuration, Some(&old_configuration))
    } else {
        ContractError::RecordNotFound {
            explanation: format!(
                "attempted to replace configuration with validation type [{}] for validator [{}] in storage, but no such configuration exists",
                configuration.get_validation_type(), configuration.validator
            ),
        }
        .to_err()
    }
}

/// Inserts a validator configuration into the contract's storage, overwriting
/// any existing validator configuration with the same key. Returns a [Result]
/// reflecting whether the insertion succeeded or not.
///
/// # Parameters
///
/// * `storage` A mutable reference to the contract's internal storage.
/// * `configuration` The validator configuration to store.
/// * `old_configuration`  The validator configuration being replaced, if it exists.
pub fn store_validator_configuration(
    storage: &mut dyn Storage,
    configuration: &ValidatorConfiguration,
    old_configuration: Option<&ValidatorConfiguration>,
) -> ContractResult<()> {
    validator_configurations()
        .replace(
            storage,
            configuration.storage_key().as_bytes(),
            Some(configuration),
            old_configuration,
        )
        .map_err(|e| ContractError::StorageError {
            message: format!("{:?}", e),
        })
}

/// Finds a validator configuration from the contract's storage by its key, returning
/// a [Result] reflecting whether the validator configuration was found or not.
///
/// # Parameters
///
/// * `storage` An immutable reference to the contract's internal storage.
/// * `key` A storage key for a validator configuration.
pub fn get_validator_configuration<S: Into<String>>(
    storage: &dyn Storage,
    key: S,
) -> ContractResult<ValidatorConfiguration> {
    validator_configurations()
        .load(storage, key.into().as_bytes())
        .map_err(|e| ContractError::StorageError {
            message: format!("{:?}", e),
        })
}

/// Finds a validator configuration from the contract's storage by its key, returning
/// an [Option] reflecting whether the validator configuration was found or not.
///
/// # Parameters
///
/// * `storage` An immutable reference to the contract's internal storage.
/// * `key` A storage key for a validator configuration.
pub fn may_get_validator_configuration<S: Into<String>>(
    storage: &dyn Storage,
    key: S,
) -> Option<ValidatorConfiguration> {
    validator_configurations()
        .may_load(storage, key.into().as_bytes())
        .unwrap_or(None)
}
