use crate::execute::create_request::create_request_for_validation;
use crate::execute::create_validation_definition::create_validation_definition;
use crate::execute::update_settings::update_settings;
use crate::instantiate::instantiate_contract;
use crate::migrate::migrate_contract;
use crate::query::get_info::query_contract_info;
use crate::query::get_request::query_request;
use crate::query::get_validation_definition::query_validation_definition;
use crate::types::core::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::util::aliases::{DepsC, DepsMutC, EntryPointResponse, QueryResult};
use cosmwasm_std::{entry_point, Env, MessageInfo};

#[entry_point]
pub fn instantiate(
    deps: DepsMutC,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> EntryPointResponse {
    instantiate_contract(deps, env, info, msg)
}

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

#[entry_point]
pub fn query(deps: DepsC, _env: Env, msg: QueryMsg) -> QueryResult {
    match msg {
        QueryMsg::QueryRequestOrder { id } => query_request(&deps, id),
        QueryMsg::QueryValidationDefinition { key } => query_validation_definition(&deps, key),
        QueryMsg::QueryContractInfo {} => query_contract_info(deps.storage),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMutC, _env: Env, msg: MigrateMsg) -> EntryPointResponse {
    match msg {
        MigrateMsg::ContractUpgrade {} => migrate_contract(deps),
    }
}
