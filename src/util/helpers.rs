use super::aliases::{ContractResult, DepsC};
use crate::{storage::contract_info::get_contract_info, types::core::error::ContractError};

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
/// # Examples
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
/// # Examples
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
