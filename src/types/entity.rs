use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct EntityDetail {
    pub address: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub home_url: Option<String>,
    pub source_url: Option<String>,
}
