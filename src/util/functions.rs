use std::{collections::HashMap, hash::Hash};

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

/// Compares two arrays and returns true if they are subsets of each other.
/// Sourced from [StackOverflow](https://stackoverflow.com/a/42748484).
///
/// # Parameters
/// `a` One of the arrays.
/// `b` The other array.
pub fn equal_sets<T>(a: &[T], b: &[T]) -> bool
where
    T: Eq + Hash,
{
    fn count<T>(items: &[T]) -> HashMap<&T, usize>
    where
        T: Eq + Hash,
    {
        let mut count = HashMap::new();
        for i in items {
            *count.entry(i).or_insert(0) += 1
        }
        count
    }
    count(a) == count(b)
}
