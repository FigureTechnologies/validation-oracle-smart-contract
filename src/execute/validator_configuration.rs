use crate::{
    storage::{
        contract_info::get_contract_info,
        entity::get_entity,
        validator_configuration::{
            get_validator_configuration, insert_validator_configuration,
            store_validator_configuration,
        },
    },
    types::{
        core::error::ContractError,
        request::validator_configuration::{
            ValidatorConfigurationCreationRequest, ValidatorConfigurationUpdateRequest,
        },
        validator_configuration::ValidatorConfiguration,
    },
    util::{
        aliases::{DepsMutC, EntryPointResponse},
        event_attributes::{EventAttributes, EventType},
        helpers::{check_funds_are_empty, get_validator_configuration_update},
    },
};

use cosmwasm_std::{Env, MessageInfo, Response};
use result_extensions::ResultExtensions;

pub fn create_new_validator_configuration(
    deps: DepsMutC,
    _env: Env,
    info: MessageInfo,
    request: ValidatorConfigurationCreationRequest,
) -> EntryPointResponse {
    // Validate the request
    check_funds_are_empty(&info)?;
    let state = get_contract_info(deps.storage)?;
    if info.sender != request.validator && info.sender != state.admin {
        return ContractError::Unauthorized {
            reason: "Must be the contract admin to create a validator configuration for a different address"
                .to_string(),
        }
        .to_err();
    }
    match get_entity(deps.storage, request.validator.clone()) {
        Ok(_) => {}
        Err(err) => {
            return ContractError::InvalidRequest {
                message: format!(
                    "No entity with the address [{}] was found in storage: {}",
                    request.validator, err
                ),
            }
            .to_err()
        }
    }
    // Store the validator configuration
    insert_validator_configuration(deps.storage, &request.clone().into())?;
    // Construct the response
    Response::new()
        .add_attributes(
            EventAttributes::new(EventType::AddValidatorConfiguration)
                .set_validation_type(&request.validation_type)
                .set_validator(&request.validator),
        )
        .to_ok()
}

pub fn update_existing_validator_configuration(
    deps: DepsMutC,
    _env: Env,
    info: MessageInfo,
    request: ValidatorConfigurationUpdateRequest,
) -> EntryPointResponse {
    // Validate the request
    check_funds_are_empty(&info)?;
    let state = get_contract_info(deps.storage)?;
    if info.sender != request.validator && info.sender != state.admin {
        return ContractError::Unauthorized {
            reason: "Must be the contract admin to create a validator configuration for a different address"
                .to_string(),
        }
        .to_err();
    }
    match request.maybe_get_new_validation_costs() {
        Some(_) => {} // TODO: Figure out a feasible way to check that the new costs are different without macro hell (thanks cosmwasm)
        None => {
            return ContractError::InvalidRequest {
                message:
                    "At least one change to the existing validator configuration must be specified"
                        .to_string(),
            }
            .to_err();
        }
    }
    let old_configuration = get_validator_configuration(deps.storage, request.storage_key())
        .map_err(|err| ContractError::InvalidRequest {
            message: format!(
                "No validator configuration for [{}] of validation type [{}] exists: {:?}",
                request.validator, request.validation_type, err
            ),
        })?;
    // Store the validator configuration
    let new_configuration = ValidatorConfiguration {
        validation_costs: request.validation_costs.unwrap(),
        validation_type: request.validation_type.clone(),
        validator: request.validator.clone(),
    };
    store_validator_configuration(deps.storage, &new_configuration, Some(&old_configuration))?;
    // Construct the response
    Response::new()
        .add_attributes(
            EventAttributes::new(EventType::UpdateValidatorConfiguration)
                .set_validation_type(&request.validation_type)
                .set_validator(&request.validator)
                .set_additional_metadata(&get_validator_configuration_update(
                    &old_configuration,
                    &new_configuration,
                )),
        )
        .to_ok()
}
