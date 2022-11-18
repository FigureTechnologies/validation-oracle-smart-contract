use crate::types::{
    validation_cost::ValidationCost, validator_configuration::ValidatorConfiguration,
};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct ValidatorConfigurationCreationRequest {
    pub validation_costs: Vec<ValidationCost>,
    pub validation_type: String,
    pub validator: Addr,
}
impl ValidatorConfigurationCreationRequest {
    pub fn get_validation_type(&self) -> &str {
        &self.validation_type
    }
    pub fn get_validation_costs(&self) -> &[ValidationCost] {
        &self.validation_costs
    }
}
impl From<ValidatorConfigurationCreationRequest> for ValidatorConfiguration {
    fn from(request: ValidatorConfigurationCreationRequest) -> Self {
        ValidatorConfiguration {
            validation_costs: request.validation_costs,
            validation_type: request.validation_type,
            validator: request.validator,
        }
    }
}

#[cw_serde]
pub struct ValidatorConfigurationUpdateRequest {
    pub validator: Addr,
    pub validation_type: String,
    pub validation_costs: Option<Vec<ValidationCost>>,
}
impl ValidatorConfigurationUpdateRequest {
    pub fn get_validation_type(&self) -> &str {
        &self.validation_type
    }
    pub fn maybe_get_new_validation_costs(&self) -> Option<&[ValidationCost]> {
        self.validation_costs.as_deref()
    }
    pub fn storage_key(&self) -> String {
        format!("{}-{}", &self.validator, &self.get_validation_type())
    }
}
