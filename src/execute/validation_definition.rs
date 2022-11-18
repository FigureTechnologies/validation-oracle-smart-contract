use crate::{
    storage::{
        contract_info::get_contract_info, validation_definition::insert_validation_definition,
    },
    types::request::validation_definition::ValidationDefinitionCreationRequest,
    util::{
        aliases::{ContractResult, DepsC, DepsMutC, EntryPointResponse},
        event_attributes::{EventAttributes, EventType},
        functions::generate_validation_definition_attribute_name,
        helpers::{check_admin_only, check_funds_are_empty},
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

    let mut messages = vec![];
    if request.bind_name.unwrap_or(true) {
        messages.push(bind_name(
            generate_validation_definition_attribute_name(
                &request.validation_type,
                get_contract_info(deps.storage)?.bind_name,
            ),
            env.contract.address,
            NameBinding::Restricted,
        )?);
    }

    Response::new()
        .add_messages(messages)
        .add_attributes(
            EventAttributes::new(EventType::AddValidationDefiniton)
                .set_validation_type(&request.validation_type),
        )
        .to_ok()
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
