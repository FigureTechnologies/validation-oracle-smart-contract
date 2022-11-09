use super::access_route::AccessRoute;

use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct AccessDefinition {
    pub owner_address: String,
    pub access_routes: Vec<AccessRoute>,
    pub definition_type: AccessDefinitionType,
}
impl AccessDefinition {
    pub fn get_owner_address(&self) -> &str {
        &self.owner_address
    }
    pub fn get_access_routes(&self) -> &[AccessRoute] {
        &self.access_routes
    }
}

#[cw_serde]
pub enum AccessDefinitionType {
    /// Indicates that the access definition was created by the requestor that onboarded the scope.
    Requestor,
    /// Indicates that the access definition was created by the validator for a scope.
    Verifier,
}
