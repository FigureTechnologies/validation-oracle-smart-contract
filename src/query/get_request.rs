use crate::{storage::request_storage::may_get_request_by_id, types::core::error::ContractError};
use cosmwasm_std::{to_binary, Binary, Deps};
use provwasm_std::ProvenanceQuery;
use result_extensions::ResultExtensions;

pub fn query_request(deps: Deps<ProvenanceQuery>, id: String) -> Result<Binary, ContractError> {
    to_binary(&may_get_request_by_id(deps.storage, id))?.to_ok()
}

#[cfg(test)]
mod tests {}
