use crate::{
    storage::request::{get_request, insert_request},
    types::{core::error::ContractError, request::validation_request::ValidationRequest},
    util::{
        aliases::{DepsMutC, EntryPointResponse},
        create_request_utilities::{form_validation_request, ValidationRequestCreationResponse},
        event_attributes::{EventAttributes, EventType},
        fees::get_custom_fee_amount_display,
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
    } = form_validation_request(&deps, &env, &info, request)?;
    // Insert the request
    insert_request(deps.storage, &request_order)?;
    // Create and return a response
    let mut response = Response::new()
        .add_messages(messages)
        .add_attributes(EventAttributes::new(EventType::AddValidationRequest))
        .set_data(to_binary(&request_order)?);
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
