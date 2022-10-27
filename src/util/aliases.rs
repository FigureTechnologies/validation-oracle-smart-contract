use crate::types::core::error::ContractError;
use cosmwasm_std::{Binary, Response};
use provwasm_std::ProvenanceMsg;

pub type ContractResult = Result<Response<ProvenanceMsg>, ContractError>;

pub type QueryResult = Result<Binary, ContractError>;
