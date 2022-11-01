use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct AccessRoute {
    /// A path to a resource that can provide underlying asset data for a scope.  Can be anything:
    /// http path, grpc, etc.
    pub route: String,
    /// An optional name parameter, allowing the creator of the route to give it a definition or
    /// to label it for external resources to identify it.
    pub name: Option<String>,
}
