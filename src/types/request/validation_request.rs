use cosmwasm_std::{Addr, Coin};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A request for validation to be performed which can be submitted in a contract
/// [execution](crate::contract::execute).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ValidationRequest {
    /// The ID of the validation request. It must be unique among the contract instance's
    /// [ValidationRequestOrder]s.
    pub id: String,
    /// A list of the Provenance scopes, each denoted by its bech32 address, that are expected
    /// to be validated in order for this request to be fulfilled.
    pub scopes: Vec<Addr>,
    /// An optional list of Provenance bech32 addresses corresponding to parties which are
    /// permitted to fulfill this request. If omitted, the contract will allow any Provenance
    /// address to accept the request as a validator.
    pub allowed_validators: Option<Vec<Addr>>,
    /// The quote the requestor is offering in exchange for completion of the request.
    pub quote: Vec<Coin>,
}
impl ValidationRequest {
    pub fn get_id(&self) -> &str {
        &self.id
    }
}

/// A request for validation which is stored as a [queriable](crate::contract::query)
/// item in the contract's [storage](crate::storage::request_storage) as the result
/// of [executing](crate::contract::execute) a [request for validation](ValidationRequest).
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ValidationRequestOrder {
    /// The ID of the validation request. It must be unique within the contract instance.
    pub id: String,
    /// The Provenance bech32 address of the requestor.
    pub owner: Addr,
    /// A list of the Provenance scopes, each denoted by its bech32 address, that are expected
    /// to be validated in order for this request to be fulfilled.
    pub scopes: Vec<Addr>,
    /// An optional list of Provenance bech32 addresses corresponding to parties which are
    /// permitted to fulfill this request. If omitted, the contract will allow any Provenance
    /// address to accept the request as a validator.
    pub allowed_validators: Option<Vec<Addr>>,
    /// The quote the requestor is offering in exchange for completion of the request.
    pub quote: Vec<Coin>,
}

/// The status of a [ValidationRequestOrder].
pub enum ValidationRequestStatus {
    /// Denotes a validation request which has been submitted but not claimed or completed by any validator.
    Requested,
    // TODO: Determine if a Pending status is desired or not and update all these docs accordingly
    /// Denotes a validation request which has been submitted and claimed by a validator for fulfillment,
    /// but has yet to have results submitted.
    Pending,
    /// Denotes a validation request which has had its results submitted.
    Fulfilled,
}
