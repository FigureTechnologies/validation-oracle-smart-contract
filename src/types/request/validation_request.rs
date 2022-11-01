use cosmwasm_std::{Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub enum ValidationRequestCreationType {
    // TODO: Make generic and move to separate file if another request type is needed
    New,
    Update {
        existing_request: Box<ValidationRequest>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ValidationRequest {
    pub id: String,
    pub scopes: Vec<Addr>,
    pub allowed_validators: Option<Vec<Addr>>,
    pub quote: Vec<Coin>,
}
impl ValidationRequest {
    pub fn get_id(&self) -> &str {
        &self.id
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ValidationRequestOrder {
    pub id: String,
    pub owner: Addr,
    pub scopes: Vec<Addr>,
    pub allowed_validators: Option<Vec<Addr>>,
    pub quote: Vec<Coin>,
}
