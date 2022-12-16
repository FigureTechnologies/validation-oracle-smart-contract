use crate::storage::contract_info::ContractInfo;
use crate::types::core::msg::InstantiateMsg;
use crate::types::entity::EntityDetail;
use crate::types::request::validation_definition::ValidationDefinitionCreationRequest;
use crate::types::validation_cost::ValidationCost;
use crate::types::validation_definition::ValidationDefinition;
use crate::types::validator_configuration::ValidatorConfiguration;
use crate::util::constants::NHASH;

use cosmwasm_std::{Addr, Coin, Uint128};
use proptest::collection::vec;
use proptest::option::of as option_of;
use proptest::prelude::any;
use proptest::prop_compose;
use proptest::sample::size_range;
use proptest::strategy::{Just, Strategy};

const STRING_WITH_NON_WHITESPACE_CHARACTER: &str = r".*\S+.*";

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
    pub fn arb_entity(address: Option<Addr>)(
        random_address in arb_addr(),
        name in option_of(".+"),
        description in option_of(".+"),
        home_url in option_of(".+"),
        source_url in option_of(".+"),
    ) -> EntityDetail {
        EntityDetail {
            address: match &address {
                Some(value) => value.clone(),
                None => random_address,
            },
            name,
            description,
            home_url,
            source_url
        }
    }
}

prop_compose! {
    pub fn arb_validation_cost(address: Option<Addr>)(
        fee in arb_coin(),
        destination in arb_entity(address),
    ) -> ValidationCost {
        ValidationCost {
            amount: fee.amount,
            denom: fee.denom,
            destination
        }
    }
}

prop_compose! {
    pub fn arb_validation_type()(validation_type in r"\S+") -> String {
        // TODO: Improve regex, see below
        validation_type
    }
}

prop_compose! {
    pub fn arb_contract_bind_name()(bind_name in r"\S+") -> String {
        // TODO: Improve regex, see below
        bind_name
    }
}

prop_compose! {
    pub fn arb_validation_definition_bind_name()(_contract_bind_name in arb_contract_bind_name()) -> String {
        todo!("remove arb parameters, implement regex for requirements here: https://docs.provenance.io/modules/name-module#normalization")
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
        validation_type in STRING_WITH_NON_WHITESPACE_CHARACTER,
        display_name in option_of(STRING_WITH_NON_WHITESPACE_CHARACTER),
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
        validation_type in arb_validation_type(),
        display_name in option_of(STRING_WITH_NON_WHITESPACE_CHARACTER),
        random_enabled in any::<bool>(),
    ) -> ValidationDefinition {
        ValidationDefinition {
            validation_type,
            display_name,
            enabled: enabled.unwrap_or(random_enabled),
        }
    }
}

prop_compose! {
    pub fn arb_validator_configuration(address: Option<Addr>)(
        validation_type in arb_validation_type(),
        // (validator, validation_costs) in match address {
        //     Some(fixed_address) => (Just(fixed_address.clone()), vec(arb_validation_cost(Some(fixed_address)), size_range(1..100))),
        //     None => arb_addr().prop_flat_map(|addr| (Just(addr), vec(arb_validation_cost(Some(addr)), size_range(1..100)))),
        // },
        // TODO: Below is a terrible workaround due to the above not working for some reason
        (validator, validation_costs) in arb_addr().prop_flat_map(move |arb_addr|
            match address.clone() {
                Some(fixed_address) => Just(fixed_address),
                None => Just(arb_addr),
            }
        ).prop_flat_map(|arb_addr| (Just(arb_addr.clone()), vec(arb_validation_cost(Some(arb_addr)), size_range(1..100)),)),
    ) -> ValidatorConfiguration {
        ValidatorConfiguration { validation_type, validation_costs, validator }
    }
}
