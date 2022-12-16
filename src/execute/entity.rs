use crate::{
    storage::{contract_info::get_contract_info, entity::insert_entity, entity::update_entity},
    types::{core::error::ContractError, entity::EntityDetail},
    util::{
        aliases::{DepsMutC, EntryPointResponse},
        event_attributes::{EventAttributes, EventType},
        helpers::get_entity_update,
    },
};

use cosmwasm_std::{Env, MessageInfo, Response};
use result_extensions::ResultExtensions;

pub fn create_new_entity(
    deps: DepsMutC,
    _env: Env,
    info: MessageInfo,
    entity: EntityDetail,
) -> EntryPointResponse {
    let state = get_contract_info(deps.storage)?;
    if info.sender != entity.address && info.sender != state.admin {
        return ContractError::Unauthorized {
            reason: "must be the contract admin to create an entity with a different address"
                .to_string(),
        }
        .to_err();
    }
    insert_entity(deps.storage, &entity)?;
    Response::new()
        .add_attributes(
            EventAttributes::new(EventType::AddEntity)
                .set_entity_addresses(&[entity.address.to_string()]),
        )
        .to_ok()
}

pub fn update_existing_entity(
    deps: DepsMutC,
    _env: Env,
    info: MessageInfo,
    entity: EntityDetail,
) -> EntryPointResponse {
    let state = get_contract_info(deps.storage)?;
    if info.sender != entity.address && info.sender != state.admin {
        return ContractError::Unauthorized { reason: "must be the contract admin to update the details of an entity with a different address".to_string() }.to_err();
    }
    let old_entity = &update_entity(deps.storage, &entity)?;
    Response::new()
        .add_attributes(
            EventAttributes::new(EventType::UpdateEntity)
                .set_additional_metadata(&get_entity_update(old_entity, &entity)), // TODO: Refactor EventAdditionalMetadata to just allow for adding JSON key-value string at will instead?
        )
        .to_ok()
}
