use super::aliases::DepsC;
use crate::storage::contract_info::{get_contract_info, ContractInfo};
use crate::types::core::error::ContractError;
use crate::util::constants::NHASH;

use cosmwasm_std::{coin, Addr, CosmosMsg};
use provwasm_std::{assess_custom_fee, MsgFeesMsgParams, ProvenanceMsg, ProvenanceMsgParams};
use result_extensions::ResultExtensions;

/// Generates a fee paid to the contract as payment for usage of the contract.
/// The fee is sent to the contract's admin address so that the admin is funded
/// for the purposes of making their own requests to the contract.
///
/// # Parameters
///
/// * `fee_type` A string description of the fee being charged
/// * `deps` An immutable dependencies object provided by the cosmwasm framework.  Allows access to useful
/// resources like the contract's internal storage and a querier to retrieve blockchain objects.
/// * `contract_addr` The bech32 Provenance address of the contract iself, to facilitate the
/// contract charging the fee to the sender of a request.
/// * `fee_calculation` A function to calculate the [nhash](NHASH) fee to charge the request
/// sender given the stored [ContractInfo].
pub fn generate_contract_fee_msg<S: Into<String>, F: Fn(&ContractInfo) -> u128>(
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

/// Generates a displayable quote of a fee being charged.
///
/// # Parameters
/// `msg` An [AssessCustomFee](MsgFeesMsgParams::AssessCustomFee).
pub fn get_custom_fee_amount_display(
    msg: &CosmosMsg<ProvenanceMsg>,
) -> Result<String, ContractError> {
    match msg {
        CosmosMsg::Custom(ProvenanceMsg {
            params: ProvenanceMsgParams::MsgFees(MsgFeesMsgParams::AssessCustomFee { amount, .. }),
            ..
        }) => format!("{}{}", amount.amount.u128(), &amount.denom).to_ok(),
        msg => ContractError::InvalidType {
            explanation: format!(
                "expected MsgFees AssessCustomFee Provenance msg but got: {:?}",
                msg
            ),
        }
        .to_err(),
    }
}

#[cfg(test)]
mod tests {}
