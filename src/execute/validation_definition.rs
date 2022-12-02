use crate::{
    storage::{
        contract_info::get_contract_info,
        validation_definition::{
            get_validation_definition, insert_validation_definition, store_validation_definition,
        },
    },
    types::{
        core::error::ContractError,
        request::validation_definition::{
            ValidationDefinitionCreationRequest, ValidationDefinitionUpdateRequest,
        },
    },
    util::{
        aliases::{ContractResult, DepsC, DepsMutC, EntryPointResponse},
        event_attributes::{EventAttributes, EventType},
        functions::generate_validation_definition_attribute_name,
        helpers::{check_admin_only, check_funds_are_empty, get_validation_definition_update},
    },
};

use cosmwasm_std::{Env, MessageInfo, Response};
use provwasm_std::{bind_name, NameBinding};
use result_extensions::ResultExtensions;

pub fn create_new_validation_definition(
    deps: DepsMutC,
    env: Env,
    info: MessageInfo,
    request: ValidationDefinitionCreationRequest,
) -> EntryPointResponse {
    // Validate the request
    validate_request(&deps.as_ref(), &info, &request)?;
    // Store the definition
    insert_validation_definition(deps.storage, &request.clone().into())?;
    // Bind the validation type as a name to the contract address, unless the request explicitly specifies not to
    let mut messages = vec![];
    if request.bind_name.unwrap_or(true) {
        messages.push(bind_name(
            // TODO: Store bind name! Even better, create its own storage map for resilience to migrations?
            generate_validation_definition_attribute_name(
                &request.validation_type,
                get_contract_info(deps.storage)?.bind_name,
            ),
            env.contract.address,
            NameBinding::Restricted,
        )?);
    }
    // Construct the response
    Response::new()
        .add_messages(messages)
        .add_attributes(
            EventAttributes::new(EventType::AddValidationDefinition)
                .set_validation_type(&request.validation_type),
        )
        .to_ok()
}

pub fn update_existing_validation_definition(
    deps: DepsMutC,
    _env: Env,
    info: MessageInfo,
    request: ValidationDefinitionUpdateRequest,
) -> EntryPointResponse {
    // TODO: Test all possible cases of an invalid request!
    // Validate the request
    check_admin_only(&deps.as_ref(), &info)?;
    check_funds_are_empty(&info)?;
    let old_definition = get_validation_definition(deps.storage, request.old_storage_key())
        .map_err(|err| ContractError::InvalidRequest {
            message: format!(
                "No validation definition with validation type [{}] exists: {:?}",
                request.old_storage_key(),
                err
            ),
        })?;
    let mut errors = vec![];
    let maybe_new_storage_key = request.maybe_get_new_storage_key();
    let new_definition = request.clone().into();
    let definition_update_metadata =
        get_validation_definition_update(&old_definition, &new_definition);
    match maybe_new_storage_key {
        Some(new_storage_key) => {
            if request.old_storage_key() == new_storage_key {
                errors.push("cannot specify a new validation type which is the same as the old validation type".to_string());
            } else if get_validation_definition(deps.storage, &new_storage_key).is_ok() {
                errors.push(format!(
                    "a validation definition with validation type [{}] already exists",
                    new_storage_key
                ));
            }
        }
        None => {
            if !definition_update_metadata.has_metadata() {
                return ContractError::InvalidRequest {
                    message: format!(
                        "No actual changes to the existing validation definition with validation type [{}] were specified",
                        request.old_storage_key(),
                    )
                }.to_err();
            }
        }
    }
    if !errors.is_empty() {
        return ContractError::InvalidRequest {
            message: errors.join(", "),
        }
        .to_err();
    }
    // Update the definition
    store_validation_definition(deps.storage, &new_definition, Some(&old_definition))?;
    // Construct the response
    Response::new()
        .add_attributes(
            EventAttributes::new(EventType::UpdateValidationDefinition)
                .set_validation_type(
                    // TODO: Should this be the old type or the new type????!? May need to expand what key-values consist of, perhaps in same way as refactor of EventAdditionalMetadata
                    &request
                        .new_validation_type
                        .unwrap_or(request.current_validation_type),
                )
                .set_additional_metadata(&definition_update_metadata),
        )
        .to_ok()
}

pub fn delete_validation_definition(
    deps: DepsMutC,
    _env: Env,
    info: MessageInfo,
    _key: String,
) -> EntryPointResponse {
    check_admin_only(&deps.as_ref(), &info)?;
    check_funds_are_empty(&info)?;
    todo!()
}

fn validate_request(
    deps: &DepsC,
    info: &MessageInfo,
    _request: &ValidationDefinitionCreationRequest,
) -> ContractResult<()> {
    check_admin_only(deps, info)?;
    check_funds_are_empty(info)?;
    // TODO: Add regex check for validation_type being a valid name if bind_name isn't false, to preempt the provenance error with a more descriptive one
    Ok(())
}
