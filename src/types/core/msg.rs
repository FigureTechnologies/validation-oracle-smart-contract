use crate::{
    storage::contract_info::ContractInfo,
    types::{
        entity::EntityDetail,
        request::{
            settings_update::SettingsUpdate,
            validation_definition::{
                ValidationDefinitionCreationRequest, ValidationDefinitionUpdateRequest,
            },
            validation_request::{
                ValidationRequest, ValidationRequestOrder, ValidationRequestUpdate,
            },
            validator_configuration::{
                ValidatorConfigurationCreationRequest, ValidatorConfigurationUpdateRequest,
            },
        },
        validation_definition::ValidationDefinition,
    },
}; // TODO: These unused import warnings may be a Rust bug

use cosmwasm_schema::{cw_serde, QueryResponses};
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
    UpdateValidationRequest {
        request: ValidationRequestUpdate,
    },
    DeleteValidationRequest {
        id: String,
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
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Option<EntityDetail>)]
    QueryEntityByAddress { address: Addr },
    #[returns(Option<ValidationDefinition>)]
    QueryValidationDefinitionByType { r#type: String },
    #[returns(Option<ValidationRequestOrder>)]
    QueryValidationRequestById { id: String },
    #[returns(Vec<ValidationRequestOrder>)]
    QueryValidationRequestByOwner { owner: Addr },
    #[returns(Vec<ValidationRequestOrder>)]
    QueryValidationRequestByValidator { validator: Addr },
    //QueryValidationResultsBy...
    //QueryValidatorConfigurationBy...
    #[returns(ContractInfo)]
    QueryContractInfo {},
}

#[cw_serde]
pub enum MigrateMsg {
    ContractUpgrade {}, // TODO: Rename, flesh out later
}
