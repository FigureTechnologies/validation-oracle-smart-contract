use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::access_route::AccessRoute;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AccessDefinition {
    pub owner_address: String,
    pub access_routes: Vec<AccessRoute>,
    pub definition_type: AccessDefinitionType,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AccessDefinitionType {
    /// Indicates that the access definition was created by the requestor that onboarded the scope.
    Requestor,
    /// Indicates that the access definition was created by the verifier for a scope.
    Verifier,
}
