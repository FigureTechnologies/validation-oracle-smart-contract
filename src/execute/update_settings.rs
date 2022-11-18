use crate::storage::contract_info::{get_contract_info, set_contract_info};
use crate::types::core::error::ContractError;
use crate::types::request::settings_update::SettingsUpdate;
use crate::util::aliases::DepsMutC;
use crate::util::event_attributes::{EventAttributes, EventType};

use cosmwasm_std::{MessageInfo, Response};
use provwasm_std::ProvenanceMsg;
use result_extensions::ResultExtensions;

pub fn update_settings(
    deps: DepsMutC,
    info: MessageInfo,
    update: SettingsUpdate,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    validate_settings_update(&update)?;
    let mut contract_info = get_contract_info(deps.storage)?;
    if info.sender != contract_info.admin {
        return ContractError::Unauthorized {
            reason: "Must be the contract admin".to_string(),
        }
        .to_err();
    }
    if !info.funds.is_empty() {
        return ContractError::InvalidFunds {
            message: "funds cannot be provided during a settings update".to_string(),
        }
        .to_err();
    }
    let mut attributes = vec![];
    if let Some(ref new_admin) = &update.new_admin_address {
        contract_info.admin = deps.api.addr_validate(new_admin)?;
        attributes.push(("new_admin_address".to_string(), new_admin.to_string()));
    }
    // Save changes to the contract information
    set_contract_info(deps.storage, &contract_info)?;
    Response::new()
        .add_attributes(EventAttributes::new(EventType::UpdateSettings))
        .add_attributes(attributes)
        .to_ok()
}

fn validate_settings_update(msg: &SettingsUpdate) -> Result<(), ContractError> {
    let mut errors = vec![];
    if let Some(ref new_admin_address) = msg.new_admin_address {
        if new_admin_address.is_empty() {
            errors.push("new_admin_address was empty");
        }
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

#[cfg(test)]
mod tests {}
