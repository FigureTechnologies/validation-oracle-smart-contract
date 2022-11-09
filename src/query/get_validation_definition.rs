use crate::{
    storage::validation_definition::may_get_validation_definition,
    util::aliases::{DepsC, QueryResult},
};

use cosmwasm_std::to_binary;
use result_extensions::ResultExtensions;

pub fn query_validation_definition(deps: &DepsC, key: String) -> QueryResult {
    to_binary(&may_get_validation_definition(deps.storage, key))?.to_ok()
}

#[cfg(test)]
mod tests {}
