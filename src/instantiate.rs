use crate::storage::contract_info::{set_contract_info, ContractInfo};
use crate::types::core::error::ContractError;
use crate::types::core::msg::InstantiateMsg;
use crate::util::aliases::{ContractResult, DepsMutC, EntryPointResponse};
use crate::util::event_attributes::{EventAttributes, EventType};
use crate::util::helpers::check_funds_are_empty;

use cosmwasm_std::{Env, MessageInfo, Response};
use provwasm_std::{bind_name, NameBinding};
use result_extensions::ResultExtensions;

/// The main functionality executed when the smart contract is first instantiated. This creates
/// the internal [ContractInfo](crate::storage::contract_info::ContractInfo) value.
///
/// # Parameters
///
/// * `deps` A mutable dependencies object provided by the cosmwasm framework.  Allows access to useful
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
        contract_info.bind_name.clone(),
        env.contract.address,
        NameBinding::Restricted,
    )?;

    Response::new()
        .add_message(bind_name_msg)
        .add_attributes(
            EventAttributes::new(EventType::InstantiateContract).set_contract_info(&contract_info),
        )
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
    if msg.bind_name.trim().is_empty() {
        errors.push("bind_name value was empty");
    }
    if msg.contract_name.trim().is_empty() {
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
    use crate::{
        instantiate::instantiate_contract,
        test::{
            arbitrary::{arb_addr, arb_instantiate_msg},
            helpers::single_attribute_for_key,
        },
        types::core::error::ContractError,
        util::constants::EVENT_TYPE_KEY,
    };

    use cosmwasm_std::testing::{mock_env, mock_info};
    use proptest::{prop_assert, prop_assert_eq, proptest};
    use provwasm_mocks::mock_dependencies;

    proptest! {
        #[test]
        fn instantiate_with_valid_data(instantiate_msg in arb_instantiate_msg(), sender in arb_addr()) {
            let mut deps = mock_dependencies(&[]);

            let response = instantiate_contract(deps.as_mut(), mock_env(), mock_info(sender.as_str(), &[]), instantiate_msg);
            // TODO: Verify if interpolation is needed in message or if the shrunken test output is sufficient
            prop_assert!(response.is_ok(), "instantiation with valid input produced an error");
            let response = response.unwrap();

            prop_assert_eq!(
                2,
                response.attributes.len(),
                "two attributes should be emitted"
            );
            prop_assert_eq!(
                "instantiate_contract",
                single_attribute_for_key(&response, EVENT_TYPE_KEY),
                "the proper event type should be emitted",
            );
        }

        #[test]
        fn instantiate_with_blank_bind_name(
            valid_instantiate_msg in arb_instantiate_msg(),
            sender in arb_addr(),
            blank_bind_name in r"\s*", // TODO: What is more conventional for regex strings, `r"\s+"` or `"\\s+"`?
        ) {
            let mut deps = mock_dependencies(&[]);

            let mut invalid_instantiate = valid_instantiate_msg.clone();
            invalid_instantiate.bind_name = blank_bind_name;
            let response = instantiate_contract(deps.as_mut(), mock_env(), mock_info(sender.as_str(), &[]), invalid_instantiate);
            prop_assert!(response.is_err(), "instantiation with invalid input unexpectedly produced no error");
            let response = response.unwrap_err();
            match response {
                ContractError::InvalidInstantiation { message } => {
                    prop_assert!(message.contains("bind_name value was empty"))
                },
                // TODO: How to check that error is of the type we want (ContractError::InvalidRequest)
                // without early panicking or doing `prop_assert(false, "message we want")`?
                error => prop_assert!(false, "instantation error was of an unexpected type: [{}]", error),
            }
        }

        #[test]
        fn instantiate_with_blank_contract_name(
            valid_instantiate_msg in arb_instantiate_msg(),
            sender in arb_addr(),
            blank_contract_name in r"\s*", // TODO: What is more conventional for regex strings, `r"\s+"` or `"\\s+"`?
        ) {
            let mut deps = mock_dependencies(&[]);

            let mut invalid_instantiate = valid_instantiate_msg.clone();
            invalid_instantiate.contract_name = blank_contract_name;
            let response = instantiate_contract(deps.as_mut(), mock_env(), mock_info(sender.as_str(), &[]), invalid_instantiate);
            prop_assert!(response.is_err(), "instantiation with invalid input unexpectedly produced no error");
            let response = response.unwrap_err();
            match response {
                ContractError::InvalidInstantiation { message } => {
                    prop_assert!(message.contains("contract_name value was empty"))
                },
                // TODO: How to check that error is of the type we want (ContractError::InvalidRequest)
                // without early panicking or doing `prop_assert(false, "message we want")`?
                error => prop_assert!(false, "instantation error was of an unexpected type: [{}]", error),
            }
        }
    }
}
