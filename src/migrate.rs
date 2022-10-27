use crate::storage::contract_info::{
    get_contract_info, set_contract_info, ContractInfo, CONTRACT_TYPE, CONTRACT_VERSION,
};
use crate::types::core::error::ContractError;
use cosmwasm_std::{to_binary, DepsMut, Response};
use provwasm_std::{ProvenanceMsg, ProvenanceQuery};
use result_extensions::ResultExtensions;
use semver::Version;

pub fn migrate_contract(
    deps: DepsMut<ProvenanceQuery>,
) -> Result<Response<ProvenanceMsg>, ContractError> {
    let mut contract_info = get_contract_info(deps.storage)?;
    check_valid_migration_target(&contract_info)?;
    contract_info.contract_version = CONTRACT_VERSION.to_string();
    set_contract_info(deps.storage, &contract_info)?;
    Response::new()
        .add_attribute("action", "migrate_contract")
        .add_attribute("new_version", CONTRACT_VERSION)
        .set_data(to_binary(&contract_info)?)
        .to_ok()
}

fn check_valid_migration_target(contract_info: &ContractInfo) -> Result<(), ContractError> {
    // Prevent other contracts from being migrated over this one
    if CONTRACT_TYPE != contract_info.contract_type {
        return ContractError::InvalidMigration {
            message: format!(
                "target migration contract type [{}] does not match stored contract type [{}]",
                CONTRACT_TYPE, contract_info.contract_type,
            ),
        }
        .to_err();
    }
    let existing_contract_version = contract_info.contract_version.parse::<Version>()?;
    let new_contract_version = CONTRACT_VERSION.parse::<Version>()?;
    // Ensure only new contract versions are migrated to
    if existing_contract_version >= new_contract_version {
        return ContractError::InvalidMigration {
            message: format!(
                "target migration contract version [{}] is too low to use. stored contract version is [{}]",
                CONTRACT_VERSION, &contract_info.contract_version,
            ),
        }
        .to_err();
    }
    ().to_ok()
}

#[cfg(test)]
mod tests {}
