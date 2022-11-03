use crate::types::request::{
    settings_update::SettingsUpdate, validation_definition::ValidationDefinitionCreationRequest,
    validation_request::ValidationRequest,
};
use cosmwasm_std::Uint128;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    pub bind_name: String,
    pub contract_name: String,
    pub create_request_nhash_fee: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    QueryValidationDefinition { key: String },
    QueryRequestOrder { id: String },
    QueryContractInfo {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum MigrateMsg {
    ContractUpgrade {}, // TODO: Flesh out later
}
