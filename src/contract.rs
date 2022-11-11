use crate::execute::create_request::create_request_for_validation;
use crate::execute::create_validation_definition::create_validation_definition;
use crate::execute::update_settings::update_settings;
use crate::instantiate::instantiate_contract;
use crate::migrate::migrate_contract;
use crate::query::contract_info::query_contract_info;
use crate::query::request::{
    query_request_by_id, query_request_by_owner, query_request_by_validator,
};
use crate::query::validation_definition::query_definition_by_type;
use crate::types::core::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::util::aliases::{DepsC, DepsMutC, EntryPointResponse, QueryResult};

use cosmwasm_std::{entry_point, Env, MessageInfo};

/// The entry point used when an external address instantiates a stored code wasm payload of this
/// contract on the Provenance Blockchain.
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
#[entry_point]
pub fn instantiate(
    deps: DepsMutC,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> EntryPointResponse {
    instantiate_contract(deps, env, info, msg)
}

/// The entry point used when an external address desires to initiate a process defined in the
/// contract. This defines the primary purposes of this contract, like requesting validation or
/// submitting validation results, as well as allowing the administrator address to make changes
/// to the contract's internal configuration.
///
/// # Parameters
///
/// * `deps` A mutable dependencies object provided by the cosmwasm framework.  Allows access to useful
/// resources like the contract's internal storage and a querier to retrieve blockchain objects.
/// * `env` An environment object provided by the cosmwasm framework.  Describes the contract's
/// details, as well as blockchain information at the time of the transaction.
/// * `info` A message information object provided by the cosmwasm framework.  Describes the sender
/// of the instantiation message, as well as the funds provided as an amount during the transaction.
/// * `msg` A custom execution message enum defined by this contract to allow multiple different
/// processes to be defined for the singular execution route entry point allowed by the
/// cosmwasm framework.
#[entry_point]
pub fn execute(deps: DepsMutC, env: Env, info: MessageInfo, msg: ExecuteMsg) -> EntryPointResponse {
    match msg {
        ExecuteMsg::CreateValidationDefinition { request } => {
            create_validation_definition(deps, env, info, request)
        }
        ExecuteMsg::RequestValidation { request } => {
            create_request_for_validation(deps, env, info, request)
        }
        ExecuteMsg::UpdateSettings { update } => update_settings(deps, info, update),
    }
}

/// The entry point used when an external address desires to retrieve information from the contract.
/// Allows access to the internal storage information, as well as scope attributes emitted by the
/// onboarding process.
///
/// # Parameters
///
/// * `deps` An immutable dependencies object provided by the cosmwasm framework.  Allows access to useful
/// resources like the contract's internal storage and a querier to retrieve blockchain objects.
/// * `_env` An environment object provided by the cosmwasm framework.  Describes the contract's
/// details, as well as blockchain information at the time of the transaction.  Unused by this
/// function, but required by cosmwasm for successful query entrypoint.
/// * `msg` A custom query message enum defined by this contract to allow multiple different results
/// to be determined for this route.
#[entry_point]
pub fn query(deps: DepsC, _env: Env, msg: QueryMsg) -> QueryResult {
    match msg {
        QueryMsg::QueryValidationDefinitionByType { r#type } => {
            query_definition_by_type(deps.storage, r#type)
        }
        QueryMsg::QueryValidationRequestById { id } => query_request_by_id(deps.storage, id),
        QueryMsg::QueryValidationRequestByOwner { owner } => {
            query_request_by_owner(deps.storage, owner)
        }
        QueryMsg::QueryValidationRequestByValidator { validator } => {
            query_request_by_validator(deps.storage, validator)
        }
        QueryMsg::QueryContractInfo {} => query_contract_info(deps.storage),
    }
}

/// The entry point used when migrating a live contract instance to a new code instance, or to
/// refresh the contract with an existing matching codebase for the purpose of running migration
/// options.
///
/// # Parameters
///
/// * `deps` A mutable dependencies object provided by the cosmwasm framework.  Allows access to useful
/// resources like the contract's internal storage and a querier to retrieve blockchain objects.
/// * `_env` An environment object provided by the cosmwasm framework.  Describes the contract's
/// details, as well as blockchain information at the time of the transaction.  Unused by this
/// function, but required by cosmwasm for successful migration entrypoint.
/// * `msg` A custom migrate message enum defined by this contract to allow multiple different
/// results of invoking the migrate endpoint.
#[entry_point]
pub fn migrate(deps: DepsMutC, _env: Env, msg: MigrateMsg) -> EntryPointResponse {
    match msg {
        MigrateMsg::ContractUpgrade {} => migrate_contract(deps),
    }
}
