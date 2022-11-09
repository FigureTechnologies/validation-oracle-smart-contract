use super::aliases::ContractResult;
use crate::types::core::error::ContractError;

use cosmwasm_std::{Addr, Api};

pub fn to_valid_address<T: Into<String>>(api: &dyn Api, address: T) -> ContractResult<Addr> {
    let input = address.into();
    api.addr_validate(&input)
        .map_err(|err| ContractError::InvalidRequest {
            message: format!(
                "The provided address '{:#?}' was invalid: {:#?}",
                input, err
            ),
        })
}

pub fn generate_validation_definition_attribute_name<T: Into<String>, U: Into<String>>(
    validation_type: T,
    base_contract_name: U,
) -> String {
    format!("{}.{}", validation_type.into(), base_contract_name.into())
}
