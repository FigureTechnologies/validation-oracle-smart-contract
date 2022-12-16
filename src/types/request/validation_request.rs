use std::fmt::{Display, Formatter, Result};

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

/// A request for validation to be performed which can be submitted in a contract
/// [execution](crate::contract::execute).
#[cw_serde]
pub struct ValidationRequest {
    /// The ID of the validation request. It must be unique among the contract instance's
    /// [ValidationRequestOrder]s.
    pub id: String,
    /// A list of the Provenance scopes, each denoted by its bech32 address, that are expected
    /// to be validated in order for this request to be fulfilled.
    pub scopes: Vec<Addr>,
    /// An optional list of bech32 addresses corresponding to parties which are
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
/// item in the contract's [storage](crate::storage::request) as the result
/// of [executing](crate::contract::execute) a [request for validation](ValidationRequest).
#[cw_serde]
pub struct ValidationRequestOrder {
    /// The ID of the validation request. It must be unique within the contract instance.
    pub id: String,
    /// The bech32 address of the requestor.
    pub owner: Addr,
    /// A list of the Provenance scopes, each denoted by its bech32 address, that are expected
    /// to be validated in order for this request to be fulfilled.
    pub scopes: Vec<Addr>,
    /// An optional list of bech32 addresses corresponding to parties which are
    /// permitted to fulfill this request. If omitted, the contract will allow any Provenance
    /// address to accept the request as a validator.
    pub allowed_validators: Option<Vec<Addr>>,
    /// The quote the requestor is offering in exchange for completion of the request.
    pub quote: Vec<Coin>,
    /// The status of the validation request.
    pub status: ValidationRequestStatus,
}
impl ValidationRequestOrder {
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_scopes(&self) -> &[Addr] {
        &self.scopes
    }
    pub fn maybe_get_allowed_validators(&self) -> Option<&[Addr]> {
        self.allowed_validators.as_deref()
    }
    pub fn get_quote(&self) -> &[Coin] {
        &self.quote
    }
}

/// An update to a request for validation to be performed which can be
/// submitted in a contract [execution](crate::contract::execute).
#[cw_serde]
pub struct ValidationRequestUpdate {
    /// The ID of the existing validation request to update. It must be
    /// unique among the contract instance's [ValidationRequestOrder]s.
    pub current_id: String,
    /// An optional new ID to use for the validation request. It must be unique among the contract instance's [ValidationRequestOrder]s.
    /// If omitted, the ID of the existing validation request will not be changed.
    pub new_id: Option<String>,
    /// An optional list of Provenance scopes, each denoted by its bech32 address, that are expected to be validated in order for
    /// this request to be fulfilled. If omitted, the scopes listed on the existing validation request will not be replaced.
    pub new_scopes: Option<Vec<Addr>>,
    /// An optional list of new bech32 addresses corresponding to parties which are permitted to fulfill this request. If omitted,
    /// the allowed validators listed on the existing validation request, whether empty or not, will not be replaced.
    pub new_allowed_validators: Option<Vec<Addr>>,
    /// An optional new quote the requestor is offering in exchange for completion of the request. If omitted,
    /// the quote listed on the existing validation request, whether empty or not, will not be replaced.
    pub new_quote: Option<Vec<Coin>>,
}
impl ValidationRequestUpdate {
    pub fn get_current_id(&self) -> &str {
        &self.current_id
    }
    pub fn maybe_get_new_id(&self) -> Option<&str> {
        self.new_id.as_deref()
    }
    pub fn get_new_id(&self) -> &str {
        self.new_id.as_deref().unwrap()
    }
    pub fn maybe_get_new_scopes(&self) -> Option<&[Addr]> {
        self.new_scopes.as_deref()
    }
    pub fn get_new_scopes(&self) -> &[Addr] {
        self.new_scopes.as_deref().unwrap()
    }
    pub fn maybe_get_new_allowed_validators(&self) -> Option<&[Addr]> {
        self.new_allowed_validators.as_deref()
    }
    pub fn get_new_allowed_validators(&self) -> &[Addr] {
        self.new_allowed_validators.as_deref().unwrap()
    }
    pub fn maybe_get_new_quote(&self) -> Option<&[Coin]> {
        self.new_quote.as_deref()
    }
    pub fn get_new_quote(&self) -> &[Coin] {
        self.new_quote.as_deref().unwrap()
    }
}

/// The status of a [ValidationRequestOrder].
#[cw_serde]
pub enum ValidationRequestStatus {
    /// Denotes a validation request which has been submitted but not claimed or completed by any validator.
    Requested,
    // TODO: Determine if a Pending status is desired or not (depends on if requests target single validator or not) and update all these docs accordingly
    /// Denotes a validation request which has been submitted and claimed by a validator for fulfillment,
    /// but has yet to have results submitted.
    Pending,
    /// Denotes a validation request which has had its results submitted.
    Fulfilled,
}
impl Display for ValidationRequestStatus {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ValidationRequestStatus::Requested => write!(f, "requested"),
            ValidationRequestStatus::Pending => write!(f, "pending"),
            ValidationRequestStatus::Fulfilled => write!(f, "fulfilled"),
        }
    }
}
