use super::sort::NO_VALIDATION_REQUEST_SORT;
use crate::{
    storage::request::{get_requests_by_owner, get_requests_by_validator, may_get_request},
    util::aliases::QueryResult,
};

use cosmwasm_std::{to_binary, Addr, Storage};
use result_extensions::ResultExtensions;

/// Queries the contract's internal [storage](crate::storage::request) for a
/// [ValidationRequestOrder](crate::types::request::validation_request::ValidationRequestOrder)
/// with the given id.
///
/// # Parameters
///
/// * `storage` An immutable reference to the contract's internal storage.
/// * `id` The ID of the validation request to search for.
pub fn query_request_by_id(storage: &dyn Storage, id: String) -> QueryResult {
    to_binary(&may_get_request(storage, id))?.to_ok()
}

/// Queries the contract's internal [storage](crate::storage::request) for
/// [ValidationRequestOrder](crate::types::request::validation_request::ValidationRequestOrder)s
/// with the given owner.
///
/// # Parameters
///
/// * `id` An ID of an owner of a validation request.
pub fn query_request_by_owner(storage: &dyn Storage, owner: Addr) -> QueryResult {
    to_binary(&get_requests_by_owner(
        storage,
        owner,
        NO_VALIDATION_REQUEST_SORT,
    ))?
    .to_ok()
}

/// Queries the contract's internal [storage](crate::storage::request) for
/// [ValidationRequestOrder](crate::types::request::validation_request::ValidationRequestOrder)s
/// with the given owner.
///
/// # Parameters
///
/// * `storage` An immutable reference to the contract's internal storage.
/// * `id` An ID of a validator requested for validation.
pub fn query_request_by_validator(storage: &dyn Storage, validator: Addr) -> QueryResult {
    to_binary(&get_requests_by_validator(
        storage,
        validator,
        NO_VALIDATION_REQUEST_SORT,
    ))?
    .to_ok()
}

#[cfg(test)]
mod tests {}
