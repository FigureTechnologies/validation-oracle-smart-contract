use crate::types::request::validation_request::{
    ValidationRequestCreationType, ValidationRequestOrder,
};
use crate::types::{core::error::ContractError, request::validation_request::ValidationRequest};
use crate::util::request_fee::generate_request_fee_msg;
use cosmwasm_std::{CosmosMsg, DepsMut, Env, MessageInfo};
use provwasm_std::{ProvenanceMsg, ProvenanceQuery};
use result_extensions::ResultExtensions;

pub struct ValidationRequestCreationResponse {
    pub request_order: ValidationRequestOrder,
    pub messages: Vec<CosmosMsg<ProvenanceMsg>>,
    pub request_fee_msg: Option<CosmosMsg<ProvenanceMsg>>,
}

pub fn form_validation_request(
    deps: &DepsMut<ProvenanceQuery>,
    env: &Env,
    info: &MessageInfo,
    request: ValidationRequest,
    creation_type: ValidationRequestCreationType,
) -> Result<ValidationRequestCreationResponse, ContractError> {
    let request_fee_msg = match &creation_type {
        ValidationRequestCreationType::New => generate_request_fee_msg(
            "request creation",
            &deps.as_ref(),
            env.contract.address.clone(),
            |c| c.create_request_nhash_fee.u128(),
        )?,
        // Updates do not charge creation fees
        ValidationRequestCreationType::Update { .. } => None,
    };
    let messages = vec![];
    let request_order = ValidationRequestOrder {
        id: request.id,
        owner: info.sender.clone(),
        scopes: request.scopes,
        allowed_validators: request.allowed_validators,
        quote: request.quote,
    };
    validate_request_order(&request_order)?;
    ValidationRequestCreationResponse {
        request_order,
        messages,
        request_fee_msg,
    }
    .to_ok()
}

pub fn validate_request_order(request_order: &ValidationRequestOrder) -> Result<(), ContractError> {
    let mut errors = vec![];
    if request_order.id.is_empty() {
        errors.push("request order is missing ID");
    }
    if request_order.owner.to_string().is_empty() {
        errors.push("request order is missing owner");
    }
    if request_order.scopes.is_empty() {
        errors.push("request order is missing a scope");
    }
    if !errors.is_empty() {
        ContractError::InvalidRequest {
            message: errors.join(", "),
        }
        .to_err()
    } else {
        Ok(())
    }
}
