use crate::{storage::entity::may_get_entity, util::aliases::QueryResult};

use cosmwasm_std::{to_binary, Addr, Storage};
use result_extensions::ResultExtensions;

/// Queries the contract's internal [storage](crate::storage::entity) for an
/// [EntityDetail](crate::types::entity::EntityDetail)
/// with the given address.
///
/// # Parameters
///
/// * `storage` An immutable reference to the contract's internal storage.
/// * `address` The bech32 Provenance address of the entity to search for.
pub fn query_entity_by_address(storage: &dyn Storage, address: Addr) -> QueryResult {
    to_binary(&may_get_entity(storage, address))?.to_ok()
}

#[cfg(test)]
mod tests {}
