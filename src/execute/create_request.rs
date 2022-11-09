use crate::{
    storage::request_storage::{get_request_by_id, insert_request},
    types::{core::error::ContractError, request::validation_request::ValidationRequest},
    util::{
        aliases::{DepsMutC, EntryPointResponse},
        create_request_utilities::{form_validation_request, ValidationRequestCreationResponse},
        provenance::get_custom_fee_amount_display,
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
    // Check if request with same ID already exists
    if get_request_by_id(deps.storage, request.get_id()).is_ok() {
        return ContractError::ExistingId {
            id: request.get_id().to_string(),
            id_type: "request".to_string(),
        }
        .to_err();
    }
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
        .add_attribute("action", "create_request")
        .add_attribute("request_id", &request_order.id)
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
