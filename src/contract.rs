use crate::execute::create_request::create_request_for_validation;
use crate::execute::update_settings::update_settings;
use crate::instantiate::instantiate_contract;
use crate::migrate::migrate_contract;
use crate::query::get_info::query_contract_info;
use crate::query::get_request::query_request;
use crate::types::core::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::util::aliases::{ContractResult, QueryResult};
use cosmwasm_std::{entry_point, Deps, DepsMut, Env, MessageInfo};
use provwasm_std::ProvenanceQuery;

#[entry_point]
pub fn instantiate(
    deps: DepsMut<ProvenanceQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult {
    instantiate_contract(deps, env, info, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut<ProvenanceQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult {
    match msg {
        ExecuteMsg::RequestValidation { request } => {
            create_request_for_validation(deps, env, info, request)
        }
        ExecuteMsg::UpdateSettings { update } => update_settings(deps, info, update),
    }
}

#[entry_point]
pub fn query(deps: Deps<ProvenanceQuery>, _env: Env, msg: QueryMsg) -> QueryResult {
    match msg {
        QueryMsg::QueryRequestOrder { id } => query_request(&deps, id),
        QueryMsg::QueryContractInfo {} => query_contract_info(deps.storage),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut<ProvenanceQuery>, _env: Env, msg: MigrateMsg) -> ContractResult {
    match msg {
        MigrateMsg::ContractUpgrade {} => migrate_contract(deps),
    }
}
