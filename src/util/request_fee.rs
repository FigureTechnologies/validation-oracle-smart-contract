use crate::storage::contract_info::{get_contract_info, ContractInfo};
use crate::types::core::error::ContractError;
use crate::util::constants::NHASH;
use cosmwasm_std::{coin, Addr, CosmosMsg};
use provwasm_std::{assess_custom_fee, ProvenanceMsg};
use result_extensions::ResultExtensions;

use super::aliases::DepsC;

pub fn generate_request_fee_msg<S: Into<String>, F: Fn(&ContractInfo) -> u128>(
    fee_type: S,
    deps: &DepsC,
    contract_addr: Addr,
    fee_calculation: F,
) -> Result<Option<CosmosMsg<ProvenanceMsg>>, ContractError> {
    let fee_type = fee_type.into();
    let contract_info = get_contract_info(deps.storage)?;
    let nhash_fee_amount = fee_calculation(&contract_info);
    // Only dispatch a fee message if the fee amount is greater than zero. Charging a fee of zero
    // means nothing
    if nhash_fee_amount > 0 {
        Some(assess_custom_fee(
            // Provenance Blockchain fees are required to be sent as either usd or nhash.  This
            // contract only supports nhash for now
            coin(nhash_fee_amount, NHASH),
            // Specify a somewhat descriptive message to ensure that signers using the Provenance
            // Blockchain wallet can understand the reason for the fee
            Some(format!("{} {} fee", &fee_type, NHASH)),
            // The contract's address must be used as the "from" value.  This does not mean that
            // the contract sends the fee, but it is required for the contract to sign and dispatch
            // the message that will charge the request sender the fee
            contract_addr,
            // Always send fees charged to the admin address.  This ensures that the admin is
            // always funded in order to make future requests
            Some(contract_info.admin),
        )?)
    } else {
        None
    }
    .to_ok()
}

#[cfg(test)]
mod tests {}
