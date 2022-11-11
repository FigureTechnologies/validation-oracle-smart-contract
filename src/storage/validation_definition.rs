use crate::{
    types::{core::error::ContractError, validation_definition::ValidationDefinition},
    util::aliases::ContractResult,
};

use cosmwasm_std::Storage;
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use result_extensions::ResultExtensions;

/// The primary key prefix for the storage of [ValidationDefinition]s in an [IndexedMap].
const NAMESPACE_VALIDATION_DEFINITIONS_PK: &str = "validation_definition";
/// The prefix in the [DefinitionIndices] for indexing [ValidationDefinition]s by their validation type.
const NAMESPACE_VALIDATION_DEFINITIONS_TYPE_IDX: &str = "validation_definition__type";

/// Defines a collection of [MultiIndex]s for storing [ValidationDefinition]s in
/// a shared primary key namespace.
pub struct DefinitionIndices<'a> {
    pub type_index: MultiIndex<'a, String, ValidationDefinition, String>,
}
impl<'a> IndexList<ValidationDefinition> for DefinitionIndices<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<ValidationDefinition>> + '_> {
        let v: Vec<&dyn Index<ValidationDefinition>> = vec![&self.type_index];
        Box::new(v.into_iter())
    }
}

/// Returns the contract's storage of validation definitions.
fn definitions<'a>() -> IndexedMap<'a, &'a [u8], ValidationDefinition, DefinitionIndices<'a>> {
    let indices = DefinitionIndices {
        type_index: MultiIndex::new(
            |_pk, request: &ValidationDefinition| request.validation_type.clone(),
            NAMESPACE_VALIDATION_DEFINITIONS_PK,
            NAMESPACE_VALIDATION_DEFINITIONS_TYPE_IDX,
        ),
    };
    IndexedMap::new(NAMESPACE_VALIDATION_DEFINITIONS_PK, indices)
}

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
    let state = definitions();
    let key = definition.storage_key();
    if let Ok(existing_definition) = state.load(storage, key.as_bytes()) {
        ContractError::RecordAlreadyExists {
            explanation: format!(
                "unique constraints violated! record with validation type '{}' already exists",
                existing_definition.validation_type
            ),
        }
        .to_err()
    } else {
        state
            .save(storage, key.as_bytes(), definition)
            .map_err(|e| ContractError::StorageError {
                message: format!("{:?}", e),
            })
    }
}

/// Finds a validation definition from the contract's storage by its key,
/// returning a [Result] reflecting whether the retrieval succeeded or not.
///
/// # Parameters
///
/// * `storage` An immutable reference to the contract's internal storage.
/// * `key` A storage key for a validation definition.
pub fn get_validation_definition<S: Into<String>>(
    storage: &dyn Storage,
    key: S,
) -> ContractResult<ValidationDefinition> {
    definitions()
        .load(storage, key.into().as_bytes())
        .map_err(|e| ContractError::StorageError {
            message: format!("{:?}", e),
        })
}

/// Finds a validation definition from the contract's storage by its key, returning
/// an [Option] reflecting whether the validation definition was found or not.
///
/// # Parameters
///
/// * `storage` An immutable reference to the contract's internal storage.
/// * `key` A storage key for a validation definition.
pub fn may_get_validation_definition<S: Into<String>>(
    storage: &dyn Storage,
    key: S,
) -> Option<ValidationDefinition> {
    definitions()
        .may_load(storage, key.into().as_bytes())
        .unwrap_or(None)
}
