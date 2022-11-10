use crate::storage::contract_info::{get_contract_info, set_contract_info, ContractInfo};
use crate::types::core::error::ContractError;
use crate::types::core::msg::InstantiateMsg;
use crate::util::aliases::{ContractResult, DepsMutC, EntryPointResponse};
use crate::util::event_attributes::{EventAttributes, EventType};
use crate::util::helpers::check_funds_are_empty;

use cosmwasm_std::{Env, MessageInfo, Response};
use provwasm_std::{bind_name, NameBinding};
use result_extensions::ResultExtensions;

/// The main functionality executed when the smart contract is first instantiated. This creates
/// the internal [contract state](crate::storage::contract_info::ContractInfo) value.
///
/// # Parameters
///
/// * `deps` A dependencies object provided by the cosmwasm framework.  Allows access to useful
/// resources like the contract's internal storage and a querier to retrieve blockchain objects.
/// * `env` An environment object provided by the cosmwasm framework.  Describes the contract's
/// details, as well as blockchain information at the time of the transaction.
/// * `info` A message information object provided by the cosmwasm framework.  Describes the sender
/// of the instantiation message, as well as the funds provided as an amount during the transaction.
/// * `msg` A custom instantiation message defined by this contract for creating the initial
/// configuration used by the contract.
pub fn instantiate_contract(
    deps: DepsMutC,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> EntryPointResponse {
    check_funds_are_empty(&info)?;
    validate_instantiate_msg(&msg)?;
    let contract_info = ContractInfo::new(
        info.sender,
        msg.bind_name,
        msg.contract_name,
        Some(msg.create_request_nhash_fee),
    );
    set_contract_info(deps.storage, &contract_info)?;

    let bind_name_msg = bind_name(
        contract_info.bind_name,
        env.contract.address,
        NameBinding::Restricted,
    )?;

    Response::new()
        .add_message(bind_name_msg)
        .add_attributes(EventAttributes::new(EventType::InstantiateContract(
            &get_contract_info(deps.storage)?,
        )))
        .to_ok()
}

/// Checks that a given contract instantation is valid.
///
/// # Parameters
///
/// * `msg` The custom instantiation message defined by this contract for creating the initial
/// configuration used by the contract.
fn validate_instantiate_msg(msg: &InstantiateMsg) -> ContractResult<()> {
    let mut errors = vec![];
    if msg.bind_name.is_empty() {
        errors.push("bind_name value was empty");
    }
    if msg.contract_name.is_empty() {
        errors.push("contract_name value was empty");
    }
    if !errors.is_empty() {
        ContractError::InvalidInstantiation {
            message: errors.join(", "),
        }
        .to_err()
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use provwasm_mocks::mock_dependencies;

    use crate::{
        test::{
            cosmos_type_helpers::single_attribute_for_key,
            mock_instantiate::{test_instantiate, TestInstantiate},
        },
        util::constants::EVENT_TYPE_KEY,
    };

    #[test]
    fn test_valid_default_instantiate() {
        let mut deps = mock_dependencies(&[]);
        let response = test_instantiate(deps.as_mut(), TestInstantiate::default())
            .expect("the default instantiate should produce a response without error");
        assert_eq!(
            2,
            response.attributes.len(),
            "a single attribute should be emitted"
        );
        assert_eq!(
            "instantiate_contract",
            single_attribute_for_key(&response, EVENT_TYPE_KEY),
            "the proper event type should be emitted",
        );
    }
}
