use crate::{
    storage::{
        contract_info::get_contract_info,
        request::{delete_request_by_id, get_request, insert_request, store_request},
    },
    types::{
        core::error::ContractError,
        request::validation_request::{
            ValidationRequest, ValidationRequestType, ValidationRequestUpdate,
        },
    },
    util::{
        aliases::{DepsMutC, EntryPointResponse},
        create_request_utilities::{form_validation_request, ValidationRequestCreationResponse},
        event_attributes::{EventAttributes, EventType},
        fees::get_custom_fee_amount_display,
        helpers::get_validation_request_update,
    },
};

use cosmwasm_std::{to_binary, Env, MessageInfo, Response};
use result_extensions::ResultExtensions;

pub fn create_request_for_validation(
    deps: DepsMutC,
    env: Env,
    info: MessageInfo,
    request: ValidationRequest,
) -> EntryPointResponse {
    // Validate the request
    if get_request(deps.storage, request.get_id()).is_ok() {
        return ContractError::ExistingId {
            id: request.get_id().to_string(),
            id_type: "request".to_string(),
        }
        .to_err();
    }
    // TODO: Should we let validation requests (...I forgot the rest, leaving this here as a note in case I remember)
    // Form the request's messages
    let ValidationRequestCreationResponse {
        request_order,
        messages,
        request_fee_msg,
    } = form_validation_request(&deps, &env, &info, request, ValidationRequestType::New)?;
    // Insert the request
    insert_request(deps.storage, &request_order)?;
    // Create and return a response
    let mut response = Response::new()
        .add_messages(messages)
        .add_attributes(EventAttributes::new(EventType::AddValidationRequest))
        .set_data(to_binary(&request_order)?); // TODO: Add set_data calls to other entry point responses
    if let Some(request_fee_msg) = request_fee_msg {
        response = response
            .add_attribute(
                "request_creation_fee_charged",
                get_custom_fee_amount_display(&request_fee_msg)?,
            )
            .add_message(request_fee_msg);
    }
    response.to_ok()
}

pub fn update_request_for_validation(
    deps: DepsMutC,
    env: Env,
    info: MessageInfo,
    request: ValidationRequestUpdate,
) -> EntryPointResponse {
    // TODO: Complete details
    // Validate the request
    let _state = get_contract_info(deps.storage)?;
    // TODO: Further validation that request actually changes something
    // if info.sender != existing_request.owner && info.sender != state.admin {
    //     return ContractError::Unauthorized {
    //         reason: "Must be the contract admin to update a validation request owned by a different address"
    //             .to_string(),
    //     }
    //     .to_err();
    // }
    let old_request = get_request(deps.storage, request.get_current_id()).map_err(|err| {
        ContractError::InvalidRequest {
            message: format!(
                "No validation request with id [{}] exists: {:?}",
                request.get_current_id(),
                err
            ),
        }
    })?;
    // TODO: Use to_owned over clone for this block? Use only accessors over direct?
    let mut errors = vec![];
    let maybe_new_storage_key = request.maybe_get_new_id();
    let creation_request = ValidationRequest {
        id: maybe_new_storage_key
            .unwrap_or_else(|| request.get_current_id())
            .to_string(),
        scopes: request
            .new_scopes
            .to_owned()
            .unwrap_or_else(|| old_request.scopes.to_owned()),
        allowed_validators: match request.new_allowed_validators.to_owned() {
            None => old_request.allowed_validators.to_owned(),
            new_allowed_validators => new_allowed_validators,
        },
        quote: match request.new_quote.to_owned() {
            None => old_request.quote.to_owned(),
            Some(new_quote) => new_quote,
        },
    };
    let ValidationRequestCreationResponse {
        request_order: new_request_order,
        messages,
        request_fee_msg,
    } = form_validation_request(
        &deps,
        &env,
        &info,
        creation_request,
        ValidationRequestType::Update,
    )?;
    // Update the existing request while continuing to validate it
    let request_update_metadata = get_validation_request_update(&old_request, &new_request_order);
    match maybe_new_storage_key {
        Some(new_storage_key) => {
            if request.get_current_id() == new_storage_key {
                errors.push("cannot specify a new ID which is the same as the old ID".to_string());
            } else if get_request(deps.storage, new_storage_key).is_ok() {
                errors.push(format!(
                    "a validation request with id [{}] already exists",
                    new_storage_key
                ));
            }
            // Create the new request
            insert_request(deps.storage, &new_request_order)?;
            // Delete the old request
            delete_request_by_id(deps.storage, request.get_current_id())?;
        }
        None => {
            if !request_update_metadata.has_metadata() {
                return ContractError::InvalidRequest {
                    message: format!(
                        "No actual changes to the existing validation request with id [{}] were specified",
                        request.get_current_id(),
                    )
                }.to_err();
            }
            // Update the existing request
            store_request(deps.storage, &new_request_order, Some(&old_request))?;
        }
    }
    if !errors.is_empty() {
        return ContractError::InvalidRequest {
            message: errors.join(", "),
        }
        .to_err();
    }
    // Create and return a response
    let mut response = Response::new()
        .add_attributes(EventAttributes::new(EventType::UpdateValidationRequest))
        // TODO: Add more attributes
        .add_messages(messages);
    if let Some(request_fee_msg) = request_fee_msg {
        response = response
            .add_attribute(
                "request_creation_fee_charged",
                get_custom_fee_amount_display(&request_fee_msg)?,
            )
            .add_message(request_fee_msg);
    }
    response.to_ok()
}

pub fn delete_request_for_validation(
    deps: DepsMutC,
    _env: Env,
    _info: MessageInfo,
    id: String,
) -> EntryPointResponse {
    // TODO: Complete
    // Validate the request
    // Delete the definition
    delete_request_by_id(deps.storage, id)?;
    // Construct the response
    Response::new()
        .add_attributes(EventAttributes::new(EventType::DeleteValidationRequest))
        .to_ok()
}
