use cosmwasm_std::{to_binary, Storage};
use result_extensions::ResultExtensions;

use crate::{storage::contract_info::get_contract_info, util::aliases::QueryResult};

pub fn query_contract_info(storage: &dyn Storage) -> QueryResult {
    to_binary(&get_contract_info(storage)?)?.to_ok()
}
