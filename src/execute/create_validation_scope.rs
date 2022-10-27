use crate::{
    types::request::validation_request::ScopeCreationRequest, util::aliases::ContractResult,
};
use cosmwasm_std::{to_binary, DepsMut, Env, MessageInfo, Response};
use provwasm_std::{write_scope, Party, PartyType, ProvenanceQuery, Scope};
use result_extensions::ResultExtensions;

pub fn create_scope_for_validation(
    _deps: DepsMut<ProvenanceQuery>,
    env: Env,
    _info: MessageInfo,
    request: ScopeCreationRequest,
) -> ContractResult {
    let validation_scope: Scope = Scope {
        scope_id: request.get_validation_scope_id().to_string(),
        specification_id: request.get_validation_scope_spec_id().to_string(),
        owners: vec![Party {
            address: request.validator.clone(),
            role: PartyType::Owner,
        }],
        data_access: vec![request.validator.clone()],
        value_owner_address: request.validator.clone(),
    };
    let messages = vec![write_scope(validation_scope, vec![env.contract.address])?];
    // Create and return a response
    let response = Response::new()
        .add_messages(messages)
        .add_attribute("action", "create_scope")
        .add_attribute("validation_scope_id", &request.validation_scope_id)
        .set_data(to_binary(&request)?);
    response.to_ok()
}
