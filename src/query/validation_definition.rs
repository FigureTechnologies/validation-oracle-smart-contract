use crate::{
    storage::validation_definition::may_get_validation_definition, util::aliases::QueryResult,
};

use cosmwasm_std::{to_binary, Storage};
use result_extensions::ResultExtensions;

/// Queries the contract's internal [storage](crate::storage::validation_definition) for
/// a [ValidationDefinition](crate::types::validation_definition::ValidationDefinition)
/// with the given type.
///
/// # Parameters
///
/// * `storage` An immutable reference to the contract's internal storage.
/// * `validation_type` The type of the validation definition to search for.
pub fn query_definition_by_type(storage: &dyn Storage, validation_type: String) -> QueryResult {
    to_binary(&may_get_validation_definition(storage, validation_type))?.to_ok()
}

#[cfg(test)]
mod tests {}
