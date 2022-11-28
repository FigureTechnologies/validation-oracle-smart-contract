use crate::storage::contract_info::ContractInfo;
use crate::types::core::msg::InstantiateMsg;
use crate::types::request::validation_definition::ValidationDefinitionCreationRequest;
use crate::types::validation_definition::ValidationDefinition;
use crate::util::constants::NHASH;

use cosmwasm_std::{Addr, Coin, Uint128};
use proptest::option::of as option_of;
use proptest::prelude::any;
use proptest::prop_compose;

prop_compose! {
    // TODO: Add random bech32 generation
    pub fn arb_addr()(address in ".+") -> Addr {
        Addr::unchecked(address)
    }
}

prop_compose! {
    pub fn arb_coin()(denom in ".+", amount in any::<u128>()) -> Coin {
        Coin::new(amount, denom)
    }
}

prop_compose! {
    pub fn arb_nhash()(amount in any::<u128>()) -> Coin {
        Coin::new(amount, NHASH)
    }
}

prop_compose! {
    pub fn arb_request_creation_nhash_fee()(request_creation_nhash_fee in any::<u128>()) -> Uint128 {
        Uint128::new(request_creation_nhash_fee)
    }
}

prop_compose! {
    // TODO: Verify what edge cases ".+" produces and if it's sufficient
    pub fn arb_contract_info(use_package_values: bool)(
        admin in arb_addr(),
        bind_name in ".+",
        contract_name in ".+",
        random_contract_type in ".+",
        random_contract_version in ".+",
        create_request_nhash_fee in arb_request_creation_nhash_fee(),
    ) -> ContractInfo {
        if use_package_values {
            ContractInfo::new(admin, bind_name, contract_name, Some(create_request_nhash_fee))
        } else {
            ContractInfo {
                admin,
                bind_name,
                contract_name,
                contract_type: random_contract_type,
                contract_version: random_contract_version,
                create_request_nhash_fee,
            }
        }
    }
}

prop_compose! {
    pub fn arb_contract_bind_name()(bind_name in r"\S+") -> String {
        bind_name
    }
}

prop_compose! {
    pub fn arb_validation_definition_bind_name()(_contract_bind_name in arb_contract_bind_name()) -> String {
        todo!("must implement regex for requirements here: https://docs.provenance.io/modules/name-module#normalization")
    }
}

prop_compose! {
    pub fn arb_instantiate_msg()(
        bind_name in arb_contract_bind_name(),
        contract_name in r"\S+",
        create_request_nhash_fee in arb_request_creation_nhash_fee(),
    ) -> InstantiateMsg {
        InstantiateMsg { bind_name, contract_name, create_request_nhash_fee }
    }
}

prop_compose! {
    pub fn arb_validation_definition_creation_request(enabled: Option<bool>, bind_name: Option<bool>)(
        validation_type in ".+",
        display_name in option_of(".+"),
        random_enabled in option_of(any::<bool>()),
        random_bind_name in option_of(any::<bool>()),
    ) -> ValidationDefinitionCreationRequest {
        ValidationDefinitionCreationRequest {
            validation_type,
            display_name,
            bind_name: match bind_name {
                None => random_bind_name,
                value => value,
            },
            enabled: match enabled {
                None => random_enabled,
                value => value,
            },
        }
    }
}

prop_compose! {
    pub fn arb_validation_definition(enabled: Option<bool>)(
        validation_type in ".+",
        display_name in option_of(".+"),
        random_enabled in any::<bool>(),
    ) -> ValidationDefinition {
        ValidationDefinition {
            validation_type,
            display_name,
            enabled: match enabled {
                Some(value) => value,
                None => random_enabled,
            },
        }
    }
}
