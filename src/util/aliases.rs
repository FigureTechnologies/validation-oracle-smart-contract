use crate::types::core::error::ContractError;
use cosmwasm_std::{Binary, Deps, DepsMut, Response};
use provwasm_std::{ProvenanceMsg, ProvenanceQuery};

/// An abbreviation of the lengthy response type for contract entrypoints.
pub type EntryPointResponse = Result<Response<ProvenanceMsg>, ContractError>;

/// An abbreviation of the response type for contract query entrypoints.
pub type QueryResult = Result<Binary, ContractError>;

/// An abbreviation of the expected response type for all contract pathways with exceptional code.
pub type ContractResult<T> = Result<T, ContractError>;

/// An abbreviation of the lengthy DepsMut<'a, T> declaration. Short for "Dependencies Mutable Contract".
pub type DepsMutC<'a> = DepsMut<'a, ProvenanceQuery>;

/// An abbreviation of the lengthy Deps<'a, T> declaration. Short for "Dependencies Contract".
pub type DepsC<'a> = Deps<'a, ProvenanceQuery>;
