use crate::types::request::{
    settings_update::SettingsUpdate, validation_definition::ValidationDefinitionCreationRequest,
    validation_request::ValidationRequest,
};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

#[cw_serde]
pub struct InstantiateMsg {
    pub bind_name: String,
    pub contract_name: String,
    pub create_request_nhash_fee: Uint128,
    // TODO: Add Option<Vec<ValidationDefinitionCreationRequest>> field
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateValidationDefinition {
        request: ValidationDefinitionCreationRequest,
    },
    RequestValidation {
        request: ValidationRequest,
    },
    UpdateSettings {
        update: SettingsUpdate,
    },
}

#[cw_serde]
pub enum QueryMsg {
    QueryValidationDefinition { key: String },
    QueryRequestOrder { id: String },
    QueryContractInfo {},
}

#[cw_serde]
pub enum MigrateMsg {
    ContractUpgrade {}, // TODO: Flesh out later
}
