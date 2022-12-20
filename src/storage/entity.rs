use crate::{
    types::{core::error::ContractError, entity::EntityDetail},
    util::aliases::ContractResult,
};

use cosmwasm_std::{Addr, Storage};
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};
use result_extensions::ResultExtensions;

/// The primary key prefix for the storage of [EntityDetail]s in an [IndexedMap].
const NAMESPACE_ENTITIES_PK: &str = "entity_detail";
/// The prefix in the [EntityDetailIndices] for indexing [EntityDetail]s by their bech32 Provenance address.
const NAMESPACE_ENTITIES_IDX: &str = "entity_detail__address";

/// Defines a collection of [MultiIndex]s for storing [EntityDetail]s in
/// a shared primary key namespace.
pub struct EntityDetailIndices<'a> {
    pub address_index: MultiIndex<'a, Addr, EntityDetail, String>,
}
impl<'a> IndexList<EntityDetail> for EntityDetailIndices<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<EntityDetail>> + '_> {
        let v: Vec<&dyn Index<EntityDetail>> = vec![&self.address_index];
        Box::new(v.into_iter())
    }
}

/// Returns the contract's storage of entity details.
fn entities<'a>() -> IndexedMap<'a, &'a [u8], EntityDetail, EntityDetailIndices<'a>> {
    let indices = EntityDetailIndices {
        address_index: MultiIndex::new(
            |_pk, entity: &EntityDetail| entity.address.clone(),
            NAMESPACE_ENTITIES_PK,
            NAMESPACE_ENTITIES_IDX,
        ),
    };
    IndexedMap::new(NAMESPACE_ENTITIES_PK, indices)
}

/// Inserts an entity into the contract's storage, returning a [Result]
/// reflecting whether the insertion succeeded or not.
///
/// # Parameters
///
/// * `storage` A mutable reference to the contract's internal storage.
/// * `entity` The entity to insert.
pub fn insert_entity(storage: &mut dyn Storage, entity: &EntityDetail) -> ContractResult<()> {
    let state = entities();
    if let Ok(existing_entity) = state.load(storage, entity.address.as_bytes()) {
        ContractError::RecordAlreadyExists {
            explanation: format!(
                "an entity with address [{}] already exists",
                existing_entity.address
            ),
        }
        .to_err()
    } else {
        store_entity(storage, entity, None)?;
        Ok(())
    }
}

/// Updates an existing entity within the contract's storage. Returns
/// the former value of the entity.
///
/// # Parameters
///
/// * `storage` A mutable reference to the contract's internal storage.
/// * `entity` The new entity detail to replace the one in storage with the same address.
pub fn update_entity(
    storage: &mut dyn Storage,
    entity: &EntityDetail,
) -> ContractResult<EntityDetail> {
    let state = entities();
    if let Ok(old_entity) = state.load(storage, entity.address.as_bytes()) {
        store_entity(storage, entity, Some(&old_entity))?;
        Ok(old_entity)
    } else {
        ContractError::RecordNotFound {
            explanation: format!(
                "attempted to replace EntityDetail with address [{}] in storage, but no entity with that address exists",
                &entity.address
            ),
        }
        .to_err()
    }
}

fn store_entity(
    storage: &mut dyn Storage,
    entity: &EntityDetail,
    old_entity: Option<&EntityDetail>,
) -> ContractResult<()> {
    entities()
        .replace(storage, entity.address.as_bytes(), Some(entity), old_entity)
        .map_err(|e| ContractError::StorageError {
            message: format!("{:?}", e),
        })
}

/// Finds an entity from the contract's storage by its address, returning
/// a [Result] reflecting whether the entity was found or not.
///
/// # Parameters
///
/// * `storage` An immutable reference to the contract's internal storage.
/// * `address` The bech32 Provenance address of an entity.
pub fn get_entity<A: Into<Addr>>(
    storage: &dyn Storage,
    address: A,
) -> ContractResult<EntityDetail> {
    entities()
        .load(storage, address.into().as_bytes())
        .map_err(|e| ContractError::StorageError {
            message: format!("{:?}", e),
        })
}

/// Finds an entity from the contract's storage by its key, returning
/// an [Option] reflecting whether the entity was found or not.
///
/// # Parameters
///
/// * `storage` An immutable reference to the contract's internal storage.
/// * `address` The bech32 Provenance address of an entity.
pub fn may_get_entity<A: Into<Addr>>(storage: &dyn Storage, address: A) -> Option<EntityDetail> {
    entities()
        .may_load(storage, address.into().as_bytes())
        .unwrap_or(None)
}

/// Deletes an entity by its address, returning a [Result]
/// reflecting whether a matching entity was found or not.
///
/// # Parameters
///
/// * `storage` A mutable reference to the contract's internal storage.
/// * `address` The bech32 Provenance address of an entity.
pub fn delete_entity_by_id<A: Into<Addr>>(
    storage: &mut dyn Storage,
    address: A,
) -> ContractResult<()> {
    let id = address.into();
    entities()
        .remove(storage, id.as_bytes())
        .map_err(|e| ContractError::StorageError {
            message: format!(
                "failed to remove EntityDetail with address [{}]: {:?}",
                id, e
            ),
        })
}
