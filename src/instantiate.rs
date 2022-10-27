use crate::storage::contract_info::{get_contract_info, set_contract_info, ContractInfo};
use crate::types::core::error::ContractError;
use crate::types::core::msg::InstantiateMsg;
use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use provwasm_std::{bind_name, NameBinding, ProvenanceMsg, ProvenanceQuery};
use result_extensions::ResultExtensions;

pub fn instantiate_contract(
    deps: DepsMut<ProvenanceQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    validate_instantiate_msg(&msg)?;
    let contract_info = ContractInfo::new(
        info.sender,
        msg.bind_name,
        msg.contract_name,
        Some(msg.create_request_nhash_fee),
    );
    set_contract_info(deps.storage, &contract_info)?;

    let bind_name_msg = bind_name(
        contract_info.bind_name,
        env.contract.address,
        NameBinding::Restricted,
    )?;

    Response::new()
        .add_message(bind_name_msg)
        .add_attribute(
            "contract_info",
            format!("{:?}", get_contract_info(deps.storage)?),
        )
        .add_attribute("action", "init")
        .to_ok()
}

fn validate_instantiate_msg(msg: &InstantiateMsg) -> Result<(), ContractError> {
    let mut errors = vec![];
    if msg.bind_name.is_empty() {
        errors.push("bind_name value was empty");
    }
    if msg.contract_name.is_empty() {
        errors.push("contract_name value was empty");
    }
    if !errors.is_empty() {
        ContractError::InvalidInstantiation {
            message: errors.join(", "),
        }
        .to_err()
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {}
