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
            |_pk, definition: &ValidationDefinition| definition.validation_type.clone(),
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
/// * `definition` The validation definition to insert.
pub fn insert_validation_definition(
    storage: &mut dyn Storage,
    definition: &ValidationDefinition,
) -> ContractResult<()> {
    let state = definitions();
    let key = definition.storage_key();
    if let Ok(existing_definition) = state.load(storage, key.as_bytes()) {
        ContractError::RecordAlreadyExists {
            explanation: format!(
                "a definition with validation type [{}] already exists",
                existing_definition.validation_type
            ),
        }
        .to_err()
    } else {
        store_validation_definition(storage, definition, None)
    }
}

/// Updates an existing validation definition within the contract's storage,
/// returning a [Result] reflecting whether the insertion succeeded or not.
///
/// # Parameters
///
/// * `storage` A mutable reference to the contract's internal storage.
/// * `definition` The new validation definition to replace the one in storage with the same key.
pub fn update_validation_definition(
    storage: &mut dyn Storage,
    definition: &ValidationDefinition,
) -> ContractResult<()> {
    let state = definitions();
    if let Ok(old_definition) = state.load(storage, definition.storage_key().as_bytes()) {
        store_validation_definition(storage, definition, Some(&old_definition))
    } else {
        ContractError::RecordNotFound {
            explanation: format!(
                "attempted to replace definition with validation type [{}] in storage, but no definition with that type exists",
                &definition.storage_key()
            ),
        }
        .to_err()
    }
}

/// Inserts a validation definition into the contract's storage, overwriting
/// any existing validation definition with the same key. Returns a [Result]
/// reflecting whether the insertion succeeded or not.
///
/// # Parameters
///
/// * `storage` A mutable reference to the contract's internal storage.
/// * `definition` The validation definition to store.
/// * `old_definition`  The validation definition being replaced, if it exists.
fn store_validation_definition(
    storage: &mut dyn Storage,
    definition: &ValidationDefinition,
    old_definition: Option<&ValidationDefinition>,
) -> ContractResult<()> {
    definitions()
        .replace(
            storage,
            definition.storage_key().as_bytes(),
            Some(definition),
            old_definition,
        )
        .map_err(|e| ContractError::StorageError {
            message: format!("{:?}", e),
        })
}

/// Finds a validation definition from the contract's storage by its key,
/// returning a [Result] reflecting whether the retrieval succeeded or not.
/// Finds a validation definition from the contract's storage by its key, returning
/// a [Result] reflecting whether the the validation definition was found or not.
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

#[cfg(test)]
mod tests {
    use super::{get_validation_definition, insert_validation_definition};
    use crate::test::arbitrary::arb_validation_definition;

    use proptest::{prop_assert, prop_assert_eq, proptest};
    use provwasm_mocks::mock_dependencies;

    proptest! {
        #[test] // TODO: Should test fns be pub?
        fn store_and_retrieve_validation_definition_with_valid_data(validation_definition in arb_validation_definition(None)) {
            let mut deps = mock_dependencies(&[]);

            let result = insert_validation_definition(deps.as_mut().storage, &validation_definition);
            prop_assert!(result.is_ok(), "inserting validation definition produced an error");

            let retrieved = get_validation_definition(&deps.storage, validation_definition.storage_key());
            prop_assert!(retrieved.is_ok(), "retrieving validation definition produced an error");
            let retrieved = retrieved.unwrap();

            prop_assert_eq!(
                validation_definition.get_validation_type(),
                retrieved.get_validation_type()
            );
            prop_assert_eq!(
                validation_definition.maybe_get_display_name(),
                retrieved.maybe_get_display_name()
            );
        }
    }
}
