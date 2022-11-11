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
    pub create_request_nhash_fee: Uint128,
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
    use provwasm_mocks::mock_dependencies;

    use crate::storage::contract_info::{
        get_contract_info, may_get_contract_info, set_contract_info, ContractInfo, CONTRACT_TYPE,
        CONTRACT_VERSION,
    };
    use crate::test::mock_instantiate::{
        default_instantiate, DEFAULT_ADMIN_ADDRESS, DEFAULT_CONTRACT_BIND_NAME,
        DEFAULT_CONTRACT_NAME,
    };
    use cosmwasm_std::{Addr, Uint128};

    #[test]
    pub fn set_and_get_contract_info_with_valid_data() {
        let mut deps = mock_dependencies(&[]);
        let result = set_contract_info(
            deps.as_mut().storage,
            &ContractInfo::new(
                Addr::unchecked(DEFAULT_ADMIN_ADDRESS),
                DEFAULT_CONTRACT_BIND_NAME.to_string(),
                DEFAULT_CONTRACT_NAME.to_string(),
                None,
            ),
        );
        match result {
            Ok(()) => {}
            result => panic!("unexpected error inserting contract info: {:?}", result),
        }

        let contract_info = get_contract_info(&deps.storage);
        match contract_info {
            Ok(contract_info) => {
                assert_eq!(Addr::unchecked("contract_admin"), contract_info.admin);
                assert_eq!(DEFAULT_CONTRACT_BIND_NAME, contract_info.bind_name);
                assert_eq!(DEFAULT_CONTRACT_NAME, contract_info.contract_name);
                assert_eq!(CONTRACT_TYPE, contract_info.contract_type);
                assert_eq!(CONTRACT_VERSION, contract_info.contract_version);
                assert_eq!(Uint128::zero(), contract_info.create_request_nhash_fee);
            }
            result => panic!("unexpected error retrieving contract info: {:?}", result),
        }
    }

    #[test]
    fn test_may_get_contract_info() {
        let mut deps = mock_dependencies(&[]);
        assert!(
            may_get_contract_info(deps.as_ref().storage).is_none(),
            "contract info should not load when it has not yet been stored",
        );
        default_instantiate(deps.as_mut()).expect("default test instantiate should succeed");
        assert!(
            may_get_contract_info(deps.as_ref().storage).is_some(),
            "contract info should be available after instantiation",
        );
    }
}
