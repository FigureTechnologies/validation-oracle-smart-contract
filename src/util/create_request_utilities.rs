use super::aliases::DepsMutC;
use super::fees::generate_contract_fee_msg;
use crate::types::request::validation_request::{
    ValidationRequestOrder, ValidationRequestStatus, ValidationRequestType,
};
use crate::types::{core::error::ContractError, request::validation_request::ValidationRequest};

use cosmwasm_std::{CosmosMsg, Env, MessageInfo};
use provwasm_std::ProvenanceMsg;
use result_extensions::ResultExtensions;

pub struct ValidationRequestCreationResponse {
    pub request_order: ValidationRequestOrder,
    pub messages: Vec<CosmosMsg<ProvenanceMsg>>,
    pub request_fee_msg: Option<CosmosMsg<ProvenanceMsg>>,
}

pub fn form_validation_request(
    deps: &DepsMutC,
    env: &Env,
    info: &MessageInfo,
    request: ValidationRequest,
    request_type: ValidationRequestType,
) -> Result<ValidationRequestCreationResponse, ContractError> {
    let request_fee_msg = match request_type {
        ValidationRequestType::New => generate_contract_fee_msg(
            "validation request creation",
            &deps.as_ref(),
            env.contract.address.clone(),
            |c| c.create_request_nhash_fee.u128(),
        )?,
        ValidationRequestType::Update => None,
    };
    let messages = vec![];
    let request_order = ValidationRequestOrder {
        id: request.id,
        owner: info.sender.clone(),
        scopes: request.scopes,
        allowed_validators: request.allowed_validators,
        quote: request.quote,
        status: ValidationRequestStatus::Requested,
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
        errors.push("request order is missing ID".to_string());
    }
    if request_order.owner.to_string().is_empty() {
        errors.push("request order is missing owner".to_string());
    }
    if request_order.scopes.is_empty() {
        errors.push("request order is missing a scope".to_string());
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
