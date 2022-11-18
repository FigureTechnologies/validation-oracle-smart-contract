use super::{
    aliases::{ContractResult, DepsC},
    event_attributes::EventAdditionalMetadata,
};
use crate::{
    storage::contract_info::{get_contract_info, ContractInfo},
    types::{
        core::error::ContractError, entity::EntityDetail,
        validator_configuration::ValidatorConfiguration,
    },
};

use cosmwasm_std::MessageInfo;
use result_extensions::ResultExtensions;

/// Ensures that only the admin of the contract can call into a route.
///
/// # Parameters
///
/// * `deps` An immutable dependencies object provided by the cosmwasm framework.  Allows access to useful
/// resources like the contract's internal storage and a querier to retrieve blockchain objects.
/// * `info` A message information object provided by the cosmwasm framework.  Describes the sender
/// of the instantiation message, as well as the funds provided as an amount during the transaction.
///
/// # Example
/// ```
/// use validation_oracle_smart_contract::util::helpers::check_admin_only;
/// use validation_oracle_smart_contract::storage::contract_info::{ContractInfo, set_contract_info};
/// use cosmwasm_std::{Addr, MessageInfo, testing::mock_info};
/// use provwasm_mocks::mock_dependencies;
///
/// let mut deps = mock_dependencies(&[]);
/// set_contract_info(
///     deps.as_mut().storage,
///     &ContractInfo::new(
///         Addr::unchecked("admin-name"),
///         "bind name".to_string(),
///         "contract name".to_string(),
///         None,
///     )
/// ).expect("expected contract info to save successfully");
/// let info = mock_info("admin-name", &[]);
/// check_admin_only(&deps.as_ref(), &info).expect("admin-name was used as the admin and should return a success");
/// ```
pub fn check_admin_only(deps: &DepsC, info: &MessageInfo) -> ContractResult<()> {
    let state = get_contract_info(deps.storage)?;
    if info.sender != state.admin {
        ContractError::Unauthorized {
            reason: "must be the contract admin".to_string(),
        }
        .to_err()
    } else {
        Ok(())
    }
}

/// Ensures that the info provided to the route does not include any funds.
///
/// # Parameters
///
/// * `info` A message information object provided by the cosmwasm framework.  Describes the sender
/// of the instantiation message, as well as the funds provided as an amount during the transaction.
///
/// # Example
/// ```
/// use validation_oracle_smart_contract::util::helpers::check_funds_are_empty;
/// use cosmwasm_std::testing::mock_info;
///
/// let info = mock_info("admin-name", &[]);
/// check_funds_are_empty(&info).expect("no coin provided in info - should be success");
/// ```
pub fn check_funds_are_empty(info: &MessageInfo) -> ContractResult<()> {
    if !info.funds.is_empty() {
        ContractError::InvalidFunds {
            message: "route requires that no funds be provided".to_string(),
        }
        .to_err()
    } else {
        Ok(())
    }
}

/// Outputs the difference between two [entities](EntityDetail) as an [EventAdditionalMetadata]
/// that can be appended to a [Response](cosmwasm_std::Response).
///
/// # Parameters
/// * `old` The former version of the entity.
/// * `new` The new version of the entity.
pub fn get_entity_update(old: &EntityDetail, new: &EntityDetail) -> EventAdditionalMetadata {
    let mut changes = EventAdditionalMetadata::new();
    if old.address != new.address {
        changes.add_metadata("old_address", old.address.to_string());
        changes.add_metadata("new_address", new.address.to_string());
    }
    match (old.name.as_deref(), new.name.as_deref()) {
        (None, None) => {}
        (None, Some(new_name)) => {
            changes.add_metadata("new_name", new_name);
        }
        (Some(old_name), None) => {
            changes.add_metadata("old_name", old_name);
        }
        (Some(old_name), Some(new_name)) => {
            if old_name != new_name {
                changes.add_metadata("old_name", old_name);
                changes.add_metadata("new_name", new_name);
            }
        }
    }
    match (old.description.as_deref(), new.description.as_deref()) {
        (None, None) => {}
        (None, Some(new_description)) => {
            changes.add_metadata("new_description", new_description);
        }
        (Some(old_description), None) => {
            changes.add_metadata("old_description", old_description);
        }
        (Some(old_description), Some(new_description)) => {
            if old_description != new_description {
                changes.add_metadata("old_description", old_description);
                changes.add_metadata("new_description", new_description);
            }
        }
    }
    match (old.home_url.as_deref(), new.home_url.as_deref()) {
        (None, None) => {}
        (None, Some(new_home_url)) => {
            changes.add_metadata("new_home_url", new_home_url);
        }
        (Some(old_home_url), None) => {
            changes.add_metadata("old_home_url", old_home_url);
        }
        (Some(old_home_url), Some(new_home_url)) => {
            if old_home_url != new_home_url {
                changes.add_metadata("old_home_url", old_home_url);
                changes.add_metadata("new_home_url", new_home_url);
            }
        }
    }
    match (old.source_url.as_deref(), new.source_url.as_deref()) {
        (None, None) => {}
        (None, Some(new_source_url)) => {
            changes.add_metadata("new_source_url", new_source_url);
        }
        (Some(old_source_url), None) => {
            changes.add_metadata("old_source_url", old_source_url);
        }
        (Some(old_source_url), Some(new_source_url)) => {
            if old_source_url != new_source_url {
                changes.add_metadata("old_source_url", old_source_url);
                changes.add_metadata("new_source_url", new_source_url);
            }
        }
    }
    changes
}

/// Outputs the difference between two [ValidatorConfiguration](ValidatorConfiguration)s as an
/// [EventAdditionalMetadata] that can be appended to a [Response](cosmwasm_std::Response).
///
/// # Parameters
/// * `old` The former version of the entity.
/// * `new` The new version of the entity.
pub fn get_validator_configuration_update(
    old: &ValidatorConfiguration,
    new: &ValidatorConfiguration,
) -> EventAdditionalMetadata {
    let mut changes = EventAdditionalMetadata::new();
    if old.validator != new.validator {
        changes.add_metadata("old_validator", old.validator.to_string());
        changes.add_metadata("new_validator", new.validator.to_string());
    }
    if old.validation_type != new.validation_type {
        changes.add_metadata("old_validation_type", old.validation_type.to_owned());
        changes.add_metadata("new_validation_type", new.validation_type.to_owned());
    }
    // TODO: Implement Display for ValidationCost once we switch to using cosmwasm Coin over split fields
    /*if old.validation_costs != new.validation_costs {
        changes.add_metadata("old_validation_costs", old.validation_costs);
        changes.add_metadata("new_validation_costs", new.validation_costs);
    }*/
    changes
}

// TODO: Remove below function if the migration entrypoint can't use it
/// Outputs the difference between two [ContractInfo]s as an [EventAdditionalMetadata]
/// that can be appended to a [Response](cosmwasm_std::Response).
///
/// # Parameters
/// * `old` The former version of the contract info.
/// * `new` The new version of the contract info.
pub fn get_contract_state_update(
    old: &ContractInfo,
    new: &ContractInfo,
) -> EventAdditionalMetadata {
    let mut changes = EventAdditionalMetadata::new();
    if old.admin != new.admin {
        changes.add_metadata("old_admin", old.admin.to_string());
        changes.add_metadata("new_admin", new.admin.to_string());
    }
    if old.bind_name != new.bind_name {
        changes.add_metadata("old_bind_name", old.bind_name.to_string());
        changes.add_metadata("new_bind_name", new.bind_name.to_string());
    }
    if old.contract_name != new.contract_name {
        changes.add_metadata("old_contract_name", old.contract_name.to_string());
        changes.add_metadata("new_contract_name", new.contract_name.to_string());
    }
    if old.contract_type != new.contract_type {
        changes.add_metadata("old_contract_type", old.contract_type.to_string());
        changes.add_metadata("new_contract_type", new.contract_type.to_string());
    }
    if old.contract_version != new.contract_version {
        changes.add_metadata("old_contract_version", old.contract_version.to_string());
        changes.add_metadata("new_contract_version", new.contract_version.to_string());
    }
    if old.create_request_nhash_fee != new.create_request_nhash_fee {
        changes.add_metadata(
            "old_create_request_nhash_fee",
            old.create_request_nhash_fee.to_string(),
        );
        changes.add_metadata(
            "new_create_request_nhash_fee",
            new.create_request_nhash_fee.to_string(),
        );
    }
    changes
}
