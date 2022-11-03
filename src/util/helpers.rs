use crate::{storage::contract_info::get_contract_info, types::core::error::ContractError};

use cosmwasm_std::MessageInfo;
use result_extensions::ResultExtensions;

use super::aliases::{ContractResult, DepsC};

pub fn check_admin_only(deps: &DepsC, info: &MessageInfo) -> ContractResult<()> {
    let state = get_contract_info(deps.storage)?;
    if info.sender != state.admin {
        ContractError::Unauthorized {
            reason: "Must be the contract admin".to_string(),
        }
        .to_err()
    } else {
        Ok(())
    }
}

pub fn check_funds_are_empty(info: &MessageInfo) -> ContractResult<()> {
    if !info.funds.is_empty() {
        ContractError::InvalidFunds {
            message: "route requires no funds be present".to_string(),
        }
        .to_err()
    } else {
        Ok(())
    }
}
