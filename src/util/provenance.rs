use crate::types::core::error::ContractError;
use cosmwasm_std::CosmosMsg;
use provwasm_std::{MsgFeesMsgParams, ProvenanceMsg, ProvenanceMsgParams};
use result_extensions::ResultExtensions;

pub fn get_custom_fee_amount_display(
    msg: &CosmosMsg<ProvenanceMsg>,
) -> Result<String, ContractError> {
    match msg {
        CosmosMsg::Custom(ProvenanceMsg {
            params: ProvenanceMsgParams::MsgFees(MsgFeesMsgParams::AssessCustomFee { amount, .. }),
            ..
        }) => format!("{}{}", amount.amount.u128(), &amount.denom).to_ok(),
        msg => ContractError::GenericError {
            message: format!(
                "expected MsgFees AssessCustomFee Provenance msg but got: {:?}",
                msg
            ),
        }
        .to_err(),
    }
}
