use crate::types::{
    entity::EntityDetail,
    request::{
        settings_update::SettingsUpdate,
        validation_definition::{
            ValidationDefinitionCreationRequest, ValidationDefinitionUpdateRequest,
        },
        validation_request::ValidationRequest,
        validator_configuration::{
            ValidatorConfigurationCreationRequest, ValidatorConfigurationUpdateRequest,
        },
    },
};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};

#[cw_serde]
pub struct InstantiateMsg {
    pub bind_name: String,
    pub contract_name: String,
    pub create_request_nhash_fee: Uint128,
    // TODO: Add Option<Vec<ValidationDefinitionCreationRequest>> field?
}

#[cw_serde]
pub enum ExecuteMsg {
    CreateEntity {
        entity: EntityDetail,
    },
    UpdateEntity {
        entity: EntityDetail,
    },
    CreateValidationDefinition {
        // TODO: Worth the effort of supporting single JSON body to create if its by contract admin anyway? (Answer: probably NOT)
        request: ValidationDefinitionCreationRequest,
    },
    UpdateValidationDefinition {
        request: ValidationDefinitionUpdateRequest,
    },
    DeleteValidationDefinition {
        validation_type: String,
    },
    RequestValidation {
        request: ValidationRequest,
    },
    //AcceptValidationRequest
    //SubmitValidationResults
    // TODO: Think about possible flows of updating a definition and updating a configuration
    CreateValidatorConfiguration {
        request: ValidatorConfigurationCreationRequest,
    },
    UpdateValidatorConfiguration {
        request: ValidatorConfigurationUpdateRequest,
    },
    //DeleteValidatorConfiguration,
    UpdateSettings {
        update: SettingsUpdate,
    },
}

#[cw_serde]
pub enum QueryMsg {
    QueryEntityByAddress { address: Addr },
    QueryValidationDefinitionByType { r#type: String },
    QueryValidationRequestById { id: String },
    QueryValidationRequestByOwner { owner: Addr },
    QueryValidationRequestByValidator { validator: Addr },
    //QueryValidationResultsBy...
    //QueryValidatorConfigurationBy...
    QueryContractInfo {},
}

#[cw_serde]
pub enum MigrateMsg {
    ContractUpgrade {}, // TODO: Flesh out later
}
