use crate::{
    storage::request_storage::may_get_request_by_id,
    util::aliases::{DepsC, QueryResult},
};

use cosmwasm_std::to_binary;
use result_extensions::ResultExtensions;

pub fn query_request(deps: &DepsC, id: String) -> QueryResult {
    to_binary(&may_get_request_by_id(deps.storage, id))?.to_ok()
}

#[cfg(test)]
mod tests {}
