use crate::{types::core::error::ContractError, util::aliases::ContractResult};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Storage, Uint128};
use cw_storage_plus::Item;

pub const CONTRACT_TYPE: &str = env!("CARGO_CRATE_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// TODO: Investigate further how namespaces work with migrations and changing the string value
/// The namespace for the storage of the [ContractInfo].
const NAMESPACE_CONTRACT_INFO: &str = concat!("contract_info_", env!("CARGO_PKG_VERSION")); // Alternative: use crate const_concat
/// The contract's storage of the singleton [ContractInfo].
const CONTRACT_INFO: Item<ContractInfo> = Item::new(NAMESPACE_CONTRACT_INFO);

/// The configuration data for the contract.
#[cw_serde]
pub struct ContractInfo {
    pub admin: Addr,
    pub bind_name: String,
    pub contract_name: String,
    pub contract_type: String,
    pub contract_version: String,
    pub create_request_nhash_fee: Uint128, // TODO: Change to map or vec to store all possible contract-imposed fees, add iter() storage accessors
}
impl ContractInfo {
    pub fn new<S1: Into<String>, S2: Into<String>>(
        admin: Addr,
        bind_name: S1,
        contract_name: S2,
        create_request_nhash_fee: Option<Uint128>,
    ) -> Self {
        Self {
            admin,
            bind_name: bind_name.into(),
            contract_name: contract_name.into(),
            contract_type: CONTRACT_TYPE.to_string(),
            contract_version: CONTRACT_VERSION.to_string(),
            create_request_nhash_fee: create_request_nhash_fee.unwrap_or_else(Uint128::zero),
        }
    }
}

pub fn set_contract_info(
    storage: &mut dyn Storage,
    contract_info: &ContractInfo,
) -> ContractResult<()> {
    CONTRACT_INFO
        .save(storage, contract_info)
        .map_err(|e| ContractError::StorageError {
            message: format!("{:?}", e),
        })
}

pub fn get_contract_info(storage: &dyn Storage) -> ContractResult<ContractInfo> {
    CONTRACT_INFO
        .load(storage)
        .map_err(|e| ContractError::StorageError {
            message: format!("{:?}", e),
        })
}

pub fn may_get_contract_info(store: &dyn Storage) -> Option<ContractInfo> {
    CONTRACT_INFO.may_load(store).unwrap_or(None)
}

#[cfg(test)]
mod tests {
    use crate::instantiate::instantiate_contract;
    use crate::storage::contract_info::{
        get_contract_info, may_get_contract_info, set_contract_info, CONTRACT_TYPE,
        CONTRACT_VERSION,
    };
    use crate::test::arbitrary::{arb_addr, arb_contract_info, arb_instantiate_msg};

    use cosmwasm_std::testing::{mock_env, mock_info};
    use proptest::{prop_assert, prop_assert_eq, proptest};
    use provwasm_mocks::mock_dependencies;

    proptest! {
        #[test]
        fn set_and_get_contract_info_with_valid_data(contract_info in arb_contract_info(true)) { // TODO: Change to individual parameters
            let mut deps = mock_dependencies(&[]);

            let result = set_contract_info(
                deps.as_mut().storage,
                &contract_info,
            );
            prop_assert!(result.is_ok(), "storing contract info produced an error");

            let fetched_contract_info = get_contract_info(&deps.storage);
            prop_assert!(fetched_contract_info.is_ok(), "retrieving contract info produced an error");
            let fetched_contract_info = fetched_contract_info.unwrap(); // TODO: Verify this won't run if the above assert fails

            prop_assert_eq!(contract_info.admin, fetched_contract_info.admin);
            prop_assert_eq!(contract_info.bind_name, fetched_contract_info.bind_name);
            prop_assert_eq!(contract_info.contract_name, fetched_contract_info.contract_name);
            prop_assert_eq!(CONTRACT_TYPE, fetched_contract_info.contract_type);
            prop_assert_eq!(CONTRACT_VERSION, fetched_contract_info.contract_version);
            prop_assert_eq!(contract_info.create_request_nhash_fee, fetched_contract_info.create_request_nhash_fee);
        }

        #[test]
        fn may_get_contract_info_if_instantiated(instantiate_msg in arb_instantiate_msg(), sender in arb_addr()) {
            let mut deps = mock_dependencies(&[]);

            assert!(
                may_get_contract_info(deps.as_ref().storage).is_none(),
                "contract info unexpectedly loaded when it has not yet been stored",
            );

            let response = instantiate_contract(deps.as_mut(), mock_env(), mock_info(sender.as_str(), &[]), instantiate_msg);
            prop_assert!(response.is_ok(), "instantiation with valid input produced an error: {}", response.unwrap_err());

            assert!(
                may_get_contract_info(deps.as_ref().storage).is_some(),
                "contract info was not available after instantiation",
            );
        }
    }
}
