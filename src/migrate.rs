use crate::storage::contract_info::{
    get_contract_info, set_contract_info, ContractInfo, CONTRACT_TYPE, CONTRACT_VERSION,
};
use crate::types::core::error::ContractError;
use crate::util::aliases::{DepsMutC, EntryPointResponse};
use crate::util::event_attributes::{EventAttributes, EventType};

use cosmwasm_std::{to_binary, Response};
use result_extensions::ResultExtensions;
use semver::Version;

// TODO: Consider how operations like binding & unbinding names must be made resilient to migrations
// TODO: The below is likely an inaccurate and incomplete implementation of migrations.
/// The main entrypoint function for running a code migration.  Referred to in the [contract file](crate::contract).
///
/// # Parameters
///
/// * `deps` A mutable dependencies object provided by cosmwasm in the migrate entrypoint.
pub fn migrate_contract(deps: DepsMutC) -> EntryPointResponse {
    let mut contract_info = get_contract_info(deps.storage)?;
    check_valid_migration_target(&contract_info)?;
    contract_info.contract_version = CONTRACT_VERSION.to_string();
    set_contract_info(deps.storage, &contract_info)?;
    Response::new()
        .add_attributes(
            EventAttributes::new(EventType::MigrateContract).set_contract_info(&contract_info),
        )
        .set_data(to_binary(&contract_info)?)
        .to_ok()
}

/// Verifies that the migration is going to a proper version and the contract name of the new wasm matches
/// the value in the Cargo.toml.
///
/// # Parameters
///
/// * `contract_info` A reference to the contract's existing stored information.
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
