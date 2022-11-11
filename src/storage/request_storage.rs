use std::cmp::Ordering;

use crate::{
    types::{core::error::ContractError, request::validation_request::ValidationRequestOrder},
    util::aliases::ContractResult,
};

use cosmwasm_std::{Order, Storage};
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use result_extensions::ResultExtensions;

/// The primary key prefix for the storage of [ValidationRequestOrder]s in an [IndexedMap].
const NAMESPACE_REQUEST_PK: &str = "request";
/// The prefix in the [RequestIndices] for indexing [ValidationRequestOrder]s by their ID.
const NAMESPACE_VALIDATION_REQUEST_ID_IDX: &str = "request__id";
/// The prefix in the [RequestIndices] for indexing [ValidationRequestOrder]s by their owner.
const NAMESPACE_VALIDATION_REQUEST_OWNER_IDX: &str = "request__owner";
/// The prefix in the [RequestIndices] for indexing [ValidationRequestOrder]s by their status.
const NAMESPACE_VALIDATION_REQUEST_STATUS_IDX: &str = "request__status";

/// Defines a collection of [MultiIndex]s for storing [ValidationRequestOrder]s in
/// a shared [primary key namespace](NAMESPACE_REQUEST_PK).
pub struct RequestIndices<'a> {
    pub id_index: MultiIndex<'a, String, ValidationRequestOrder, String>,
    pub owner_index: MultiIndex<'a, String, ValidationRequestOrder, String>,
    pub status_index: MultiIndex<'a, String, ValidationRequestOrder, String>,
}
impl<'a> IndexList<ValidationRequestOrder> for RequestIndices<'a> {
    fn get_indexes(
        &'_ self,
    ) -> Box<dyn Iterator<Item = &'_ dyn Index<ValidationRequestOrder>> + '_> {
        let v: Vec<&dyn Index<ValidationRequestOrder>> =
            vec![&self.id_index, &self.owner_index, &self.status_index];
        Box::new(v.into_iter())
    }
}

/// Returns the contract's storage of validation requests.
pub fn requests<'a>() -> IndexedMap<'a, &'a [u8], ValidationRequestOrder, RequestIndices<'a>> {
    let indices = RequestIndices {
        id_index: MultiIndex::new(
            |_pk, request: &ValidationRequestOrder| request.id.clone(),
            NAMESPACE_REQUEST_PK,
            NAMESPACE_VALIDATION_REQUEST_ID_IDX,
        ),
        owner_index: MultiIndex::new(
            |_pk, request: &ValidationRequestOrder| request.owner.clone().to_string(),
            NAMESPACE_REQUEST_PK,
            NAMESPACE_VALIDATION_REQUEST_OWNER_IDX,
        ),
        status_index: MultiIndex::new(
            |_pk, request: &ValidationRequestOrder| request.status.to_string(),
            NAMESPACE_REQUEST_PK,
            NAMESPACE_VALIDATION_REQUEST_STATUS_IDX,
        ),
    };
    IndexedMap::new(NAMESPACE_REQUEST_PK, indices)
}

/// Inserts a validation request into the contract's storage.
///
/// # Parameters
///
/// * `storage` A mutable reference to the contract's internal storage.
/// * `request` The validation request to insert.
pub fn insert_request(
    storage: &mut dyn Storage,
    request: &ValidationRequestOrder,
) -> ContractResult<()> {
    let state = requests();
    if let Ok(existing_request) = state.load(storage, request.id.as_bytes()) {
        return ContractError::StorageError {
            message: format!("a request with id [{}] already exists", existing_request.id),
        }
        .to_err();
    }
    store_request(storage, request)
}

/// Updates an existing validation request within the contract's storage.
///
/// # Parameters
///
/// * `storage` A mutable reference to the contract's internal storage.
/// * `request` The new validation request to replace the one in storage with the same ID.
pub fn update_request(
    storage: &mut dyn Storage,
    request: &ValidationRequestOrder,
) -> ContractResult<()> {
    let state = requests();
    if state.load(storage, request.id.as_bytes()).is_ok() {
        delete_request_by_id(storage, &request.id)?;
        store_request(storage, request)
    } else {
        ContractError::StorageError {
            message: format!(
                "attempted to replace request with id [{}] in storage, but no request with that id existed",
                &request.id
            ),
        }
        .to_err()
    }
}

/// Inserts a validation request into the contract's storage, overwriting
/// any existing validation request with the same ID.
///
/// # Parameters
///
/// * `storage` A mutable reference to the contract's internal storage.
/// * `request` The validation request to store.
fn store_request(
    storage: &mut dyn Storage,
    request: &ValidationRequestOrder,
) -> ContractResult<()> {
    requests()
        .replace(storage, request.id.as_bytes(), Some(request), None)?
        .to_ok()
}

/// Finds a validation request by its ID, returning an [Option]
/// reflecting whether a matching request was found or not.
///
/// # Parameters
///
/// * `storage` A mutable reference to the contract's internal storage.
/// * `id` The ID of the validation request to search for.
pub fn may_get_request_by_id<S: Into<String>>(
    storage: &dyn Storage,
    id: S,
) -> Option<ValidationRequestOrder> {
    requests()
        .may_load(storage, id.into().as_bytes())
        .unwrap_or(None)
}

/// Finds a validation request by its ID, returning a [Result]
/// reflecting whether a matching request was found or not.
///
/// # Parameters
///
/// * `storage` A mutable reference to the contract's internal storage.
/// * `id` The ID of the validation request to search for.
pub fn get_request_by_id<S: Into<String>>(
    storage: &dyn Storage,
    id: S,
) -> ContractResult<ValidationRequestOrder> {
    let id = id.into();
    requests()
        .load(storage, id.as_bytes())
        .map_err(|e| ContractError::StorageError {
            message: format!(
                "failed to find ValidationRequestOrder by id [{}]: {:?}",
                id, e
            ),
        })
}

/// Finds all validation requests made by a given Provenance address.
///
/// # Parameters
///
/// * `storage` A reference to the contract's internal storage.
/// * `owner` The bech32 Provenance address of a requestor.
/// * `compare` An optional function to sort the results by.
pub fn get_requests_by_owner<
    S: Into<String>,
    C: FnMut(&ValidationRequestOrder, &ValidationRequestOrder) -> Ordering,
>(
    storage: &dyn Storage,
    owner: S,
    compare: Option<C>,
) -> Vec<ValidationRequestOrder> {
    let mut requests: Vec<ValidationRequestOrder> = requests()
        .idx
        .owner_index
        .prefix(owner.into())
        .range(storage, None, None, Order::Ascending)
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap().1)
        .collect();
    if let Some(comparator) = compare {
        requests.sort_by(comparator);
    }
    requests
}

/// Finds all validation requests with a given status.
///
/// # Parameters
///
/// * `storage` A reference to the contract's internal storage.
/// * `status` A validation request status.
/// * `compare` An optional function to sort the results by.
pub fn get_requests_by_status<
    S: Into<String>,
    C: FnMut(&ValidationRequestOrder, &ValidationRequestOrder) -> Ordering,
>(
    storage: &dyn Storage,
    status: S,
    compare: Option<C>,
) -> Vec<ValidationRequestOrder> {
    let mut requests: Vec<ValidationRequestOrder> = requests()
        .idx
        .status_index
        .prefix(status.into())
        .range(storage, None, None, Order::Ascending)
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap().1)
        .collect();
    if let Some(comparator) = compare {
        requests.sort_by(comparator);
    }
    requests
}

/// Deletes a validation request by its ID, returning a [Result]
/// reflecting whether a matching request was found or not.
///
/// # Parameters
///
/// * `storage` A mutable reference to the contract's internal storage.
/// * `id` The ID of the validation request to search for.
pub fn delete_request_by_id<S: Into<String>>(
    storage: &mut dyn Storage,
    id: S,
) -> ContractResult<()> {
    let id = id.into();
    requests()
        .remove(storage, id.as_bytes())
        .map_err(|e| ContractError::StorageError {
            message: format!(
                "failed to remove ValidationRequestOrder by id [{}]: {:?}",
                id, e
            ),
        })?;
    ().to_ok()
}
