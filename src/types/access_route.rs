use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct AccessRoute {
    /// A path to a resource that can provide underlying asset data for a scope.  Can be anything:
    /// http path, grpc, etc.
    pub route: String,
    /// An optional name parameter, allowing the creator of the route to give it a definition or
    /// to label it for external resources to identify it.
    pub name: Option<String>,
}
impl AccessRoute {
    pub fn get_route(&self) -> &str {
        &self.route
    }
    pub fn maybe_get_name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    pub fn get_name(&self) -> &str {
        self.name.as_ref().unwrap()
    }
}
